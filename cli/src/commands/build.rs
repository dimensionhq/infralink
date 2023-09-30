use colored::Colorize;
use miette::Result;
use nixpacks::nixpacks::{
    builder::docker::DockerBuilderOptions, plan::generator::GeneratePlanOptions,
};

use types::{architecture::Architecture, config::InfrastructureConfiguration};

pub async fn execute() -> Result<()> {
    // Load the current configuration
    let configuration = InfrastructureConfiguration::load::<&str>(None);

    // Check if the user's directory doesn't already have a Dockerfile they want to use
    if !std::path::Path::new("./Dockerfile").exists() {
        let name = Some(format!(
            "infralink-build-{}-{}",
            std::env::current_dir()
                .unwrap()
                .file_name()
                .unwrap()
                .to_str()
                .unwrap(),
            configuration
                .app
                .architecture
                .clone()
                .unwrap_or(Architecture::Arm64)
        ));

        let start = std::time::Instant::now();

        // First, let's generate a build plan using Nixpacks.
        let build_plan =
            nixpacks::generate_build_plan("./", vec![], &GeneratePlanOptions::default()).unwrap();

        if let Some(providers) = build_plan.providers {
            let providers_list = providers.join(", ");
            let mut noun = "app".to_string();

            if !providers_list.is_empty() {
                let length = providers.len();

                if length > 1 {
                    noun.push('s');
                }
            }

            println!(
                "Detected a {} {}. Building for {}.",
                providers_list.bright_yellow(),
                noun,
                configuration
                    .app
                    .architecture
                    .as_ref()
                    .unwrap_or(&Architecture::X86)
                    .to_string()
                    .bright_cyan(),
            )
        }

        let result = nixpacks::create_docker_image(
            "./",
            vec![],
            &GeneratePlanOptions::default(),
            &DockerBuilderOptions {
                name: name.clone(),
                cpu_quota: configuration
                    .build
                    .as_ref()
                    .unwrap()
                    .max_vcpu
                    .map(|max_vcpu| (max_vcpu * 100_000).to_string()),
                memory: configuration
                    .build
                    .as_ref()
                    .unwrap()
                    .max_memory
                    .map(|max_memory| format!("{}m", max_memory)),
                ..Default::default()
            },
        )
        .await;

        if result.is_ok() {
            println!(
                "Successfully built {} image in {:.2} seconds.",
                name.as_ref().unwrap(),
                start.elapsed().as_secs_f32()
            );
        } else {
            println!("Failed to build {} image.", name.as_ref().unwrap()); // todo: in the future, use ai to help with debugging what went wrong
        }
    }

    Ok(())
}

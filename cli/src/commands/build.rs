use miette::Result;
use nixpacks::nixpacks::{
    builder::docker::DockerBuilderOptions, plan::generator::GeneratePlanOptions,
};

pub async fn execute() -> Result<()> {
    // Check if the user's directory doesn't already have a Dockerfile
    if !std::path::Path::new("./Dockerfile").exists() {
        nixpacks::create_docker_image(
            "./",
            vec![],
            &GeneratePlanOptions::default(),
            &DockerBuilderOptions {
                name: Some("nixpacks".to_string()),
                ..Default::default()
            },
        )
        .await
        .unwrap();
    }

    Ok(())
}

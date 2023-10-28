use bollard::auth::DockerCredentials;
use miette::Result;
use types::config::InfrastructureConfiguration;

pub async fn execute() -> Result<()> {
    // Load the current configuration
    let configuration = InfrastructureConfiguration::load::<&str>(None);

    // Read ~/.infralink/[app-name]/registry.toml
    let credentials = configuration.registry();

    // Build the image, given the image name
    super::build::execute(credentials.name.clone()).await?;

    // Push the image to the registry given the image name
    let docker = bollard::Docker::connect_with_local_defaults().unwrap();

    let options = bollard::image::PushImageOptions {
        tag: credentials.tag,
    };

    let _ = docker.push_image(
        &credentials.name,
        Some(options),
        Some(DockerCredentials {
            username: Some(credentials.username),
            password: Some(credentials.password),
            ..Default::default()
        }),
    );

    Ok(())
}

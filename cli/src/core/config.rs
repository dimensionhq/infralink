use serde::{Deserialize, Serialize};

use crate::models::{cloud_provider::CloudProvider, region::Region};

#[derive(Serialize, Deserialize)]
pub struct InfrastructureConfiguration {
    // name of the user's app
    pub app: App,
}

#[derive(Serialize, Deserialize)]
pub struct App {
    pub name: String,
    pub cloud_provider: CloudProvider,
    pub region: Region,
}

impl InfrastructureConfiguration {
    pub fn builder() -> InfrastructureConfigurationBuilder {
        InfrastructureConfigurationBuilder::new()
    }

    pub fn save<P: ToString>(&self, path: Option<P>) {
        let path = path
            .map(|p| p.to_string())
            .unwrap_or_else(|| String::from("./infra.toml"));

        let file = std::fs::File::create(path).unwrap();

        let configuration = toml::to_string_pretty(self).unwrap();

        std::io::Write::write_all(&mut std::io::BufWriter::new(file), configuration.as_bytes())
            .unwrap();
    }
}

pub struct InfrastructureConfigurationBuilder {
    // name of the user's app
    app_name: String,
    // cloud provider the user wants to deploy to
    cloud_provider: CloudProvider,
    // region the user wants to deploy to
    region: Region,
}

impl InfrastructureConfigurationBuilder {
    pub fn new() -> Self {
        Self {
            app_name: String::new(),
            cloud_provider: CloudProvider::None,
            region: Region::None,
        }
    }

    pub fn with_app_name(mut self, app_name: String) -> Self {
        self.app_name = app_name;
        self
    }

    pub fn with_cloud_provider(mut self, cloud_provider: CloudProvider) -> Self {
        self.cloud_provider = cloud_provider;
        self
    }

    pub fn with_region(mut self, region: Region) -> Self {
        self.region = region;
        self
    }

    pub fn build(self) -> InfrastructureConfiguration {
        InfrastructureConfiguration {
            app: App {
                name: self.app_name,
                cloud_provider: self.cloud_provider,
                region: self.region,
            },
        }
    }
}

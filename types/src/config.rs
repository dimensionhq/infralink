use keyring::Entry;
use serde::{Deserialize, Serialize};

use super::{architecture::Architecture, cloud_provider::CloudProvider, region::Region};

#[derive(Serialize, Deserialize, Debug)]
pub struct InfrastructureConfiguration {
    // high-level information about the user's app
    pub app: App,

    // information about the user's build and configuration
    pub build: Build,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct App {
    pub name: String,
    pub cloud_provider: CloudProvider,
    pub region: String,
    pub architecture: Option<Architecture>,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct Build {
    pub max_vcpu: Option<u32>,
    pub max_memory: Option<u32>,
}

impl InfrastructureConfiguration {
    pub fn builder() -> InfrastructureConfigurationBuilder {
        InfrastructureConfigurationBuilder::new()
    }

    pub fn load<P: ToString>(path: Option<P>) -> Self {
        let path = path
            .map(|p| p.to_string())
            .unwrap_or_else(|| String::from("./infra.toml"));

        toml::from_str(&std::fs::read_to_string(path).unwrap()).unwrap()
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
    // architecture of the user's app
    architecture: Option<Architecture>,
    // maximum vcpu to be used for the build
    build_with_max_vcpu: Option<u32>,
    // max mem that can be used for the build, in megabytes
    build_with_max_memory: Option<u32>,
}

impl InfrastructureConfigurationBuilder {
    pub fn new() -> Self {
        Self {
            app_name: String::new(),
            cloud_provider: CloudProvider::None,
            region: Region::None,
            architecture: None,
            build_with_max_vcpu: None,
            build_with_max_memory: None,
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

    pub fn with_architecture(mut self, architecture: Architecture) -> Self {
        self.architecture = Some(architecture);

        self
    }

    pub fn build_with_max_vcpu(mut self, max_vcpu: u32) -> Self {
        self.build_with_max_vcpu = Some(max_vcpu);

        self
    }

    pub fn build_with_max_memory(mut self, max_memory: u32) -> Self {
        self.build_with_max_memory = Some(max_memory);

        self
    }

    pub fn build(self) -> InfrastructureConfiguration {
        InfrastructureConfiguration {
            app: App {
                name: self.app_name,
                cloud_provider: self.cloud_provider,
                region: self.region.code().unwrap(),
                architecture: self.architecture,
            },
            build: Build {
                max_vcpu: self.build_with_max_vcpu,
                max_memory: self.build_with_max_memory,
            },
        }
    }
}

#[derive(Serialize, Deserialize, Debug)]
pub enum InternalConfiguration {
    Aws(InternalAWSConfiguration),
    Gcp,
    None,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct InternalAWSConfiguration {
    pub credentials: AWSCredentials,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct AWSCredentials {
    pub access_key_id: String,
    #[serde(skip)]
    pub secret_access_key: String,
}

impl InternalAWSConfiguration {
    pub fn new(access_key_id: String) -> Self {
        Self {
            credentials: AWSCredentials {
                access_key_id,
                secret_access_key: String::new(),
            },
        }
    }

    pub fn exists(app_name: String) -> bool {
        let path = dirs::home_dir()
            .unwrap()
            .join(".infralink/")
            .join(app_name)
            .join("aws.toml");

        path.exists()
    }

    pub fn load(app_name: String) -> Self {
        let path = dirs::home_dir().unwrap().join(".infralink/").join(app_name);

        let file_path = path.join("aws.toml");

        let mut config: InternalAWSConfiguration =
            toml::from_str(&std::fs::read_to_string(file_path).unwrap()).unwrap();

        let entry = Entry::new("infralink", &config.credentials.access_key_id).unwrap();

        config.credentials.secret_access_key = entry.get_password().unwrap();

        config
    }

    pub fn save(&self, app_name: String) {
        let path = dirs::home_dir().unwrap().join(".infralink/").join(app_name);

        if !path.exists() {
            std::fs::create_dir_all(&path).unwrap();
        }

        let file_path = path.join("aws.toml");
        let file = std::fs::File::create(&file_path).unwrap();

        let configuration = toml::to_string_pretty(self).unwrap();

        std::io::Write::write_all(&mut std::io::BufWriter::new(file), configuration.as_bytes())
            .unwrap();

        #[cfg(unix)]
        {
            use std::os::unix::fs::PermissionsExt;

            // Set permissions to 600 (read/write for owner, no access for others) on Unix-like systems
            let mut permissions = std::fs::metadata(&file_path).unwrap().permissions();
            permissions.set_mode(0o600);
            std::fs::set_permissions(&file_path, permissions).unwrap();
        }
    }
}

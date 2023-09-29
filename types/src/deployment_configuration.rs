use super::region::AwsRegion;

pub struct DeploymentConfiguration {
    pub regions: Vec<AwsRegion>,
    pub control_plane_specs: Option<(u32, f32)>,
    pub spot_control_plane: bool,
    pub node_specs: (u32, f32),
    pub storage_size_gb: f64,
    pub node_count: f64,
    pub outbound_data_gb: u64,
}

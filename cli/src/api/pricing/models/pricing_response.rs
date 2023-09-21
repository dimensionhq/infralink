use serde::Deserialize;

use super::{
    block_storage_response::BlockStorageResponse,
    external_transfer_response::ExternalDataTransferResponse,
    inter_region_transfer_response::InterRegionDataTransferResponse,
    on_demand_response::OnDemandResponse, spot_response::SpotResponse,
};

#[derive(Clone, Debug, Deserialize)]
pub struct PricingResponse {
    pub data: Data,
}

#[derive(Clone, Debug, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Data {
    pub control_plane_on_demand: Option<Vec<OnDemandResponse>>,
    pub node_on_demand: Option<Vec<OnDemandResponse>>,
    pub on_demand: Option<Vec<OnDemandResponse>>,
    pub spot: Option<Vec<SpotResponse>>,
    pub inter_region_data_transfer: Option<Vec<InterRegionDataTransferResponse>>,
    pub external_data_transfer: Option<Vec<ExternalDataTransferResponse>>,
    pub block_storage: Option<Vec<BlockStorageResponse>>,
}

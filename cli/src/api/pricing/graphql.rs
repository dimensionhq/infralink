use miette::{IntoDiagnostic, Result};
use reqwest::Client;
use serde_json::json;

use crate::api::pricing::models::{
    block_storage_query::BlockStorageQuery, external_transfer_query::ExternalDataTransferQuery,
    inter_region_transfer_query::InterRegionDataTransferQuery, on_demand_query::OnDemandQuery,
    pricing_response::PricingResponse, spot_query::SpotQuery,
};

pub struct PricingQuery {
    query: String,
}

const API_URL: &str = "https://pricing.infralink.io/graphql";

fn client() -> Result<Client> {
    reqwest::Client::builder()
        .use_rustls_tls()
        .build()
        .into_diagnostic()
}

impl PricingQuery {
    pub fn start() -> Self {
        Self {
            query: String::from("query {"),
        }
    }

    pub fn with_on_demand(
        &mut self,
        alias: Option<String>,
        on_demand_options: OnDemandQuery,
    ) -> &mut Self {
        if let Some(alias) = alias {
            self.query
                .push_str(format!("{alias}: onDemand(request: {{").as_str());
        } else {
            self.query.push_str("onDemand(request: {");
        }

        if let Some(instance_types) = on_demand_options.instance_types {
            self.query.push_str("instanceTypes: [");
            for instance_type in instance_types {
                self.query.push_str(&format!("\"{}\",", instance_type));
            }
            self.query.push_str("],");
        }

        if let Some(max_memory) = on_demand_options.max_memory {
            self.query.push_str(&format!("maxMemory: {},", max_memory));
        }

        if let Some(max_price_per_hour) = on_demand_options.max_price_per_hour {
            self.query
                .push_str(&format!("maxPricePerHour: {},", max_price_per_hour));
        }

        if let Some(max_vcpu) = on_demand_options.max_vcpu {
            self.query.push_str(&format!("maxVcpu: {},", max_vcpu));
        }

        if let Some(min_memory) = on_demand_options.min_memory {
            self.query.push_str(&format!("minMemory: {},", min_memory));
        }

        if let Some(min_price_per_hour) = on_demand_options.min_price_per_hour {
            self.query
                .push_str(&format!("minPricePerHour: {},", min_price_per_hour));
        }

        if let Some(min_vcpu) = on_demand_options.min_vcpu {
            self.query.push_str(&format!("minVcpu: {},", min_vcpu));
        }

        if let Some(regions) = on_demand_options.regions {
            self.query.push_str("regions: [");
            for region in regions {
                self.query.push_str(&format!("\"{}\",", region));
            }
            self.query.push_str("],");
        }

        if let Some(sort_by) = on_demand_options.sort_by {
            self.query.push_str(&format!("sortBy: \"{}\",", sort_by));
        }

        if let Some(sort_order) = on_demand_options.sort_order {
            self.query
                .push_str(&format!("sortOrder: \"{}\",", sort_order));
        }

        if let Some(limit) = on_demand_options.limit {
            self.query.push_str(&format!("limit: {},", limit));
        }

        self.query.push_str("}) {");

        self.query.push_str("architecture");
        self.query.push_str(" instanceType");
        self.query.push_str(" memory");
        self.query.push_str(" pricePerHour");
        self.query.push_str(" region");
        self.query.push_str(" vcpuCount");

        self.query.push_str("}");

        self
    }

    pub fn with_spot(&mut self, alias: Option<String>, spot_options: SpotQuery) -> &mut Self {
        if let Some(alias) = alias {
            self.query
                .push_str(format!("{alias}: spot(request: {{").as_str());
        } else {
            self.query.push_str(" spot(request: {");
        }
        if let Some(availability_zones) = spot_options.availability_zones {
            self.query.push_str("availabilityZones: [");
            for availability_zone in availability_zones {
                self.query.push_str(&format!("\"{}\",", availability_zone));
            }
            self.query.push_str("],");
        }

        if let Some(instance_types) = spot_options.instance_types {
            self.query.push_str("instanceTypes: [");
            for instance_type in instance_types {
                self.query.push_str(&format!("\"{}\",", instance_type));
            }
            self.query.push_str("],");
        }

        if let Some(max_price_per_hour) = spot_options.max_price_per_hour {
            self.query
                .push_str(&format!("maxPricePerHour: {},", max_price_per_hour));
        }

        if let Some(min_price_per_hour) = spot_options.min_price_per_hour {
            self.query
                .push_str(&format!("minPricePerHour: {},", min_price_per_hour));
        }

        if let Some(regions) = spot_options.regions {
            self.query.push_str("regions: [");
            for region in regions {
                self.query.push_str(&format!("\"{}\",", region));
            }
            self.query.push_str("],");
        }

        if let Some(sort_by) = spot_options.sort_by {
            self.query.push_str(&format!("sortBy: \"{}\",", sort_by));
        }

        if let Some(sort_order) = spot_options.sort_order {
            self.query
                .push_str(&format!("sortOrder: \"{}\",", sort_order));
        }

        if let Some(limit) = spot_options.limit {
            self.query.push_str(&format!("limit: {},", limit));
        }

        self.query.push_str("}) {");

        self.query.push_str("instanceType");
        self.query.push_str(" pricePerHour");
        self.query.push_str(" region");
        self.query.push_str(" availabilityZone");

        self.query.push_str("}");

        self
    }

    pub fn with_inter_region_data_transfer(
        &mut self,
        inter_region_options: InterRegionDataTransferQuery,
    ) -> &mut Self {
        self.query.push_str(" interRegionDataTransfer(request: {");

        if let Some(from_region_code) = inter_region_options.from_region_code {
            self.query
                .push_str(&format!("fromRegionCode: \"{}\",", from_region_code));
        }

        if let Some(to_region_code) = inter_region_options.to_region_code {
            self.query
                .push_str(&format!("toRegionCode: \"{}\",", to_region_code));
        }

        if let Some(sort_by) = inter_region_options.sort_by {
            self.query.push_str(&format!("sortBy: \"{}\",", sort_by));
        }

        if let Some(sort_order) = inter_region_options.sort_order {
            self.query
                .push_str(&format!("sortOrder: \"{}\",", sort_order));
        }

        self.query.push_str("}) {");

        self.query.push_str("fromRegionCode");
        self.query.push_str(" pricePerGb");
        self.query.push_str(" toRegionCode");

        self.query.push_str("}");

        self
    }

    pub fn with_external_data_transfer(
        &mut self,
        external_data_transfer_options: ExternalDataTransferQuery,
    ) -> &mut Self {
        self.query.push_str(" externalDataTransfer(request: {");

        if let Some(from_region_code) = external_data_transfer_options.from_region_code {
            self.query
                .push_str(&format!("fromRegionCode: \"{}\",", from_region_code));
        }

        if let Some(sort_by) = external_data_transfer_options.sort_by {
            self.query.push_str(&format!("sortBy: \"{}\",", sort_by));
        }

        if let Some(sort_order) = external_data_transfer_options.sort_order {
            self.query
                .push_str(&format!("sortOrder: \"{}\",", sort_order));
        }

        if let Some(start_range) = external_data_transfer_options.start_range {
            self.query
                .push_str(&format!("startRange: {},", start_range));
        }

        self.query.push_str("}) {");

        self.query.push_str("endRange");
        self.query.push_str(" fromRegionCode");
        self.query.push_str(" pricePerGb");
        self.query.push_str(" startRange");

        self.query.push_str("}");

        self
    }

    pub fn with_block_storage(&mut self, block_storage_options: BlockStorageQuery) -> &mut Self {
        self.query.push_str(" blockStorage(request: {");

        if let Some(regions) = block_storage_options.regions {
            self.query.push_str("regions: [");
            for region in regions {
                self.query.push_str(&format!("\"{}\",", region));
            }
            self.query.push_str("],");
        }

        if let Some(sort_by) = block_storage_options.sort_by {
            self.query.push_str(&format!("sortBy: \"{}\",", sort_by));
        }

        if let Some(sort_order) = block_storage_options.sort_order {
            self.query
                .push_str(&format!("sortOrder: \"{}\",", sort_order));
        }

        if let Some(storage_media) = block_storage_options.storage_media {
            self.query
                .push_str(&format!("storageMedia: \"{}\",", storage_media));
        }

        if let Some(volume_api_name) = block_storage_options.volume_api_name {
            self.query
                .push_str(&format!("volumeApiName: \"{}\",", volume_api_name));
        }

        self.query.push_str("}) {");

        self.query.push_str("pricePerGbMonth");
        self.query.push_str(" region");
        self.query.push_str(" storageMedia");
        self.query.push_str(" volumeApiName");

        self.query.push_str("}");

        self
    }

    pub async fn execute(&self) -> Result<PricingResponse> {
        let client = client()?;

        let response = client
            .post(API_URL)
            .json(&json!({
                "query": self.query,
            }))
            .send()
            .await
            .into_diagnostic()?
            .json::<PricingResponse>()
            .await
            .into_diagnostic()?;

        Ok(response)
    }

    pub fn end(&mut self) -> &mut Self {
        self.query.push_str("}");

        self
    }

    pub fn print(&self) {
        println!("{}", self.query);
    }
}

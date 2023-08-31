use std::collections::HashMap;

use serde::{Deserialize, Serialize};

use crate::types::force_f32::ForceF32;

#[derive(Serialize, Deserialize, Clone, Debug)]
pub struct BulkPricingResponse {
    pub products: Option<HashMap<String, Product>>,
    pub terms: Option<Terms>,
}

#[derive(Serialize, Deserialize, Clone, Debug)]
#[serde(rename_all = "camelCase")]
pub struct Product {
    pub product_family: String,
    pub sku: String,
    pub attributes: Option<Attribute>,
}

#[derive(Serialize, Deserialize, Clone, Debug)]
#[serde(rename_all = "camelCase")]
pub struct Attribute {
    pub instance_type: Option<String>,
    pub vcpu: Option<ForceF32>,
    pub memory: Option<String>,
    pub storage: Option<String>,
    pub physical_processor: Option<String>,
    pub storage_media: Option<String>,
    pub volume_api_name: Option<String>,
}

#[derive(Serialize, Deserialize, Clone, Debug)]
pub struct Terms {
    #[serde(rename = "OnDemand")]
    pub on_demand: HashMap<String, HashMap<String, OnDemandTerms>>,
}

#[derive(Serialize, Deserialize, Clone, Debug)]
#[serde(rename_all = "camelCase")]
pub struct OnDemandTerms {
    pub sku: String,
    pub price_dimensions: HashMap<String, PriceDimension>,
}

#[derive(Serialize, Deserialize, Clone, Debug)]
#[serde(rename_all = "camelCase")]
pub struct PriceDimension {
    pub description: Option<String>,
    pub price_per_unit: PricePerUnit,
}

#[derive(Serialize, Deserialize, Clone, Debug)]
pub struct PricePerUnit {
    #[serde(rename = "USD")]
    pub usd: Option<ForceF32>,
}

#[derive(Clone, Debug)]
pub struct OnDemandInstance {
    pub region: String,
    pub instance_name: String,
    pub vcpu_count: f32,
    pub memory: f32,
    pub arch: String,
    pub price_per_hour: f32,
    pub storage: String,
}

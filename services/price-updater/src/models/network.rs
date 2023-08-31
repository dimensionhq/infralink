use std::collections::HashMap;

use serde::{Deserialize, Serialize};

use super::on_demand_pricing::PricePerUnit;

#[derive(Debug, Deserialize, Serialize)]
pub struct DataTransferResponse {
    pub products: HashMap<String, Product>,
    pub terms: Terms,
}

#[derive(Debug, Deserialize, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct Product {
    pub sku: String,
    pub attributes: Option<ProductAttributes>,
}

#[derive(Debug, Deserialize, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct ProductAttributes {
    pub transfer_type: Option<String>,
    pub from_region_code: Option<String>,
    pub to_region_code: Option<String>,
}

#[derive(Debug, Deserialize, Serialize)]
pub struct Terms {
    #[serde(rename = "OnDemand")]
    pub on_demand: HashMap<String, HashMap<String, OnDemandTerm>>,
}

#[derive(Debug, Deserialize, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct OnDemandTerm {
    pub price_dimensions: std::collections::HashMap<String, PriceDimension>,
}

#[derive(Debug, Deserialize, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct PriceDimension {
    pub rate_code: String,
    pub description: String,
    pub price_per_unit: PricePerUnit,
}

#[derive(Debug, Deserialize, Serialize)]
pub struct InterRegionPrice {
    pub from_region_code: String,
    pub to_region_code: String,
    pub price_per_gb: f32,
}

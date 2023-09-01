use crate::models::external_data_transfer_request::ExternalDataTransferRequest;
use crate::models::inter_region_data_transfer_request::InterRegionDataTransferRequest;
use crate::models::on_demand_request::OnDemandRequest;
use crate::models::spot_request::SpotRequest;
use crate::models::storage_request::StorageRequest;

pub fn validate_on_demand_request(request: OnDemandRequest) -> bool {
    // Validate sort_by
    if let Some(sort_by) = request.sort_by {
        if sort_by != "price_per_hour" && sort_by != "memory" && sort_by != "vcpu_count" {
            return false;
        }
    }

    // Validate sort_order
    if let Some(sort_order) = request.sort_order {
        if sort_order != "asc" && sort_order != "desc" {
            return false;
        }
    }

    true
}

pub fn validate_spot_request(request: SpotRequest) -> bool {
    // Validate sort_by
    if let Some(sort_by) = request.sort_by {
        if sort_by != "price_per_hour" {
            return false;
        }
    }

    // Validate sort_order
    if let Some(sort_order) = request.sort_order {
        if sort_order != "asc" && sort_order != "desc" {
            return false;
        }
    }

    true
}

pub fn validate_inter_region_transfer_request(request: InterRegionDataTransferRequest) -> bool {
    // Validate sort_by
    if let Some(sort_by) = request.sort_by {
        if sort_by != "price_per_gb" {
            return false;
        }
    }

    // Validate sort_order
    if let Some(sort_order) = request.sort_order {
        if sort_order != "asc" && sort_order != "desc" {
            return false;
        }
    }

    true
}

pub fn validate_external_data_transfer_request(request: ExternalDataTransferRequest) -> bool {
    // Validate sort_by
    if let Some(sort_by) = request.sort_by {
        if sort_by != "price_per_gb" {
            return false;
        }
    }

    // Validate sort_order
    if let Some(sort_order) = request.sort_order {
        if sort_order != "asc" && sort_order != "desc" {
            return false;
        }
    }

    true
}

pub fn validate_storage_request(request: StorageRequest) -> bool {
    // Validate sort_by
    if let Some(sort_by) = request.sort_by {
        if sort_by != "price_per_gb_month" {
            return false;
        }
    }

    // Validate sort_order
    if let Some(sort_order) = request.sort_order {
        if sort_order != "asc" && sort_order != "desc" {
            return false;
        }
    }

    true
}

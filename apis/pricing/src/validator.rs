use crate::constants::regions::AWS_REGIONS;
use crate::models::on_demand_request::OnDemandRequest;
use crate::models::spot_request::SpotRequest;

pub fn validate_on_demand_request(request: OnDemandRequest) -> bool {
    // Validate regions
    if let Some(regions) = request.regions {
        if !regions.is_empty() {
            for region in regions {
                if !AWS_REGIONS.contains(&region.as_str()) {
                    return false;
                }
            }
        }
    }

    // Validate instance types
    if let Some(instance_types) = request.instance_types {
        if !instance_types.is_empty() {
            for instance_type in instance_types {
                if !crate::constants::instance_types::INSTANCE_TYPES
                    .contains(&instance_type.as_str())
                {
                    return false;
                }
            }
        }
    }

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
    // Validate regions
    if let Some(regions) = request.regions {
        if !regions.is_empty() {
            for region in regions {
                if !AWS_REGIONS.contains(&region.as_str()) {
                    return false;
                }
            }
        }
    }

    // Validate availability zone (it should be {region}-{number from a-z}, like us-east-1a)
    if let Some(availability_zones) = request.availability_zones {
        if !availability_zones.is_empty() {
            for availability_zone in availability_zones {
                let split: Vec<&str> = availability_zone.split('-').collect();

                if split[2].len() != 2 {
                    return false;
                }

                // combine the first split, second split, and first character of the last split
                let region = format!("{}-{}-{}", split[0], split[1], &split[2][0..1]);

                if !AWS_REGIONS.contains(&region.as_str()) {
                    return false;
                }

                // check if the last split is a letter
                if !split[2].chars().last().unwrap().is_alphabetic() {
                    return false;
                }
            }
        }
    }

    // Validate instance types
    if let Some(instance_types) = request.instance_types {
        if !instance_types.is_empty() {
            for instance_type in instance_types {
                if !crate::constants::instance_types::SPOT_INSTANCE_TYPES
                    .contains(&instance_type.as_str())
                {
                    return false;
                }
            }
        }
    }

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

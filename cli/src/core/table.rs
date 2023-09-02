use aws_sdk_account::types::RegionOptStatus;
use comfy_table::{modifiers::UTF8_ROUND_CORNERS, presets::UTF8_FULL, Cell, Color, Table};
use linked_hash_map::LinkedHashMap;

use crate::models::region::AwsRegion;

pub async fn render_region_pricing(regions: LinkedHashMap<AwsRegion, RegionOptStatus>) {
    let mut table = Table::new();

    table.set_header(vec![
        "name",
        "code",
        "small deployment",
        "large deployment",
        "status",
    ]);

    let mut regions_vec: Vec<(AwsRegion, RegionOptStatus)> = regions.into_iter().collect();

    let small_deployment_prices = crate::core::math::calculate_cheapest_deployment(
        regions_vec
            .iter()
            .map(|(region, _)| region.clone())
            .collect(),
    )
    .await;

    let large_deployment_prices = crate::core::math::calculate_large_deployment(
        regions_vec
            .iter()
            .map(|(region, _)| region.clone())
            .collect(),
    )
    .await;

    // sort the regions by the lowest cost spot instance price
    regions_vec.sort_by(|(region1, _), (region2, _)| {
        let price1 = small_deployment_prices.get(region1).unwrap();
        let price2 = small_deployment_prices.get(region2).unwrap();

        price1
            .partial_cmp(price2)
            .unwrap_or(std::cmp::Ordering::Equal)
    });

    for (region, status) in &regions_vec {
        let display_name = region.display_name();
        let code = region.code();

        let small_deployment_price = small_deployment_prices.get(region).unwrap();

        let large_deployment_price = large_deployment_prices.get(region).unwrap();

        table.add_row(
            vec![
                Cell::new(display_name).fg(Color::Blue),
                Cell::new(code).fg(Color::Cyan),
                Cell::new(small_deployment_price.to_string()).fg(Color::Green),
                Cell::new(large_deployment_price.to_string()).fg(Color::Green),
                match status {
                    RegionOptStatus::Enabled => Cell::new("enabled").fg(Color::Green),
                    RegionOptStatus::EnabledByDefault => Cell::new("enabled").fg(Color::Green),
                    RegionOptStatus::Disabled => Cell::new("opt-in").fg(Color::Yellow),
                    _ => Cell::new("unavailable").fg(Color::Red),
                },
            ]
            .into_iter(),
        );
    }

    table
        .load_preset(UTF8_FULL)
        .apply_modifier(UTF8_ROUND_CORNERS);

    println!("{}", table);
}

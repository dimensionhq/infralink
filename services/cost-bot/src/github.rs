use indexmap::IndexMap;
use reqwest::header::USER_AGENT;
use serde_json::json;

pub async fn write_comment_to_ref(comment: String, commit_ref: &str, repository_name: &str) -> u64 {
    let token = std::env::var("GITHUB_TOKEN").expect("GITHUB_TOKEN not set");
    let parts: Vec<&str> = repository_name.split('/').collect();
    if parts.len() != 2 {
        eprintln!("Invalid repository name");
    }
    let (owner, repo) = (parts[0], parts[1]);

    let client = reqwest::Client::new();

    let response = client
        .post(&format!(
            "https://api.github.com/repos/{}/{}/commits/{}/comments",
            owner, repo, commit_ref
        ))
        .header(USER_AGENT, "Infralink Cost Bot")
        .bearer_auth(token)
        .json(&serde_json::json!({ "body": comment }))
        .send()
        .await
        .expect("Failed to send request");

    if response.status() != 201 {
        panic!("Failed to write comment to commit");
    }

    let json: serde_json::Value = response.json().await.unwrap();
    

    json["id"]
        .as_u64()
        .ok_or("Failed to get comment ID")
        .unwrap()
}

pub async fn comment(
    previous_breakdown: Option<IndexMap<String, IndexMap<String, f64>>>,
    breakdowns: &IndexMap<String, IndexMap<String, f64>>,
    commit_ref: &str,
    repository_name: &str,
) -> u64 {
    // Calculate the total monthly cost for all apps
    let total_cost: f64 = breakdowns
        .iter()
        .map(|(_, costs)| costs.values().sum::<f64>())
        .sum();

    // Calculate the previous total cost, if available
    let prev_total_cost: Option<f64> = previous_breakdown.as_ref().map(|prev| {
        prev.iter()
            .map(|(_, costs)| costs.values().sum::<f64>())
            .sum()
    });

    // Calculate the total cost change and percentage, if previous data is available
    let (total_cost_diff, _total_percent_diff) = if let Some(prev_total) = prev_total_cost {
        let diff = total_cost - prev_total;
        let percent_diff = (diff / prev_total) * 100.0;
        let sign = if diff > 0.0 { "⬆️" } else { "⬇️" };
        (
            format!(" (Change: {} {:.1}%)", sign, percent_diff.abs()),
            true,
        )
    } else {
        (String::from(""), false)
    };

    // Create the header and summary
    let header = format!(
      "## Cost Breakdown 📊\n\n## Summary\n\n- **Total Monthly Cost**: ${:.2}{}\n\n---\n\n## Detailed Breakdown",
      total_cost, total_cost_diff
    );

    // Generate the markdown for each app
    let markdown = breakdowns
      .iter()
      .map(|(app, costs)| {
          let prev_costs = previous_breakdown.as_ref().and_then(|prev| prev.get(app));
          let has_change_for_app = costs.iter().any(|(service, cost)| {
              if service != "Total" && service != "Data Transfer" {
                  prev_costs.map_or(false, |prev| {
                      prev.get(service).map_or(false, |prev_cost| (cost - prev_cost).abs() > 1e-9)
                  })
              } else {
                  false
              }
          });

          let table_header = if has_change_for_app {
              "| Service            | Cost  | Change (%) |\n|--------------------|-------|------------|"
          } else {
              "| Service            | Cost  |\n|--------------------|-------|"
          };

          let costs_table = costs
              .iter()
              .filter_map(|(service, cost)| {
                  if service != "Total" && service != "Data Transfer" {
                      let change_str = if let Some(prev_cost) = prev_costs.and_then(|prev| prev.get(service)) {
                          let diff = cost - prev_cost;
                          let percent = (diff / prev_cost) * 100.0;
                          if diff.abs() > 1e-9 {
                              let sign = if diff > 0.0 { "⬆️" } else { "⬇️" };
                              Some(format!("{} {:.1}%", sign, percent.abs()))
                          } else {
                              None
                          }
                      } else {
                          None
                      };

                      let change_column = if has_change_for_app {
                          format!("| {}", change_str.unwrap_or_default())
                      } else {
                          "".to_string()
                      };

                      Some(format!("| {} | ${:.2} {}", service, cost, change_column))
                  } else {
                      None
                  }
              })
              .collect::<Vec<_>>()
              .join("\n");

          let total_cost: f64 = costs.values().sum();
          let total_cost_diff = if let Some(prev_costs) = prev_costs {
              let prev_total: f64 = prev_costs.values().sum();
              let diff = total_cost - prev_total;
              if diff.abs() > 1e-9 {
                  let sign = if diff > 0.0 { "⬆️" } else { "⬇️" };
                  format!("| {} {:.1}%", sign, ((diff / prev_total) * 100.0).abs())
              } else {
                  "| -".to_string()
              }
          } else {
              "| -".to_string()
          };

          let total_line = format!("| **Total** | **${:.2}** {}", total_cost, total_cost_diff);

          format!(
              "### {}\n\n{}\n{}\n{}",
              app, table_header, costs_table, total_line
          )
      })
      .collect::<Vec<_>>()
      .join("\n\n");

    let full_markdown = format!("{}\n\n{}", header, markdown);

    // Write the markdown to a comment on the commit
    let id = write_comment_to_ref(full_markdown, commit_ref, repository_name).await;

    println!(
        "Successfully completed cost analysis for {} @ {}",
        repository_name, commit_ref
    );

    id
}

pub async fn mark_check_run(
    repository_name: &str,
    commit_ref: &str,
    status: &str,
    title: &str,
    summary: &str,
    comment_id: u64,
) {
    let token = std::env::var("GITHUB_TOKEN").expect("GITHUB_TOKEN not set");
    let client = reqwest::Client::new();

    let url = format!(
        "https://api.github.com/repos/{}/check-runs",
        repository_name
    );

    let body = json!({
        "name": title,
        "head_sha": commit_ref,
        "status": "completed",
        "conclusion": status,
        "details_url": format!("https://github.com/{}/commit/{}#commitcomment-{}", repository_name, commit_ref, comment_id),
        "output": {
            "title": title,
            "summary": summary
        }
    });

    let response = client
        .post(&url)
        .header(USER_AGENT, "Cost-Bot")
        .header("Authorization", format!("Bearer {}", token))
        .json(&body)
        .send()
        .await
        .expect("Failed to mark check run");

    if response.status() != 201 {
        panic!("Failed to mark check run");
    }
}

use std::str::FromStr;

use crate::models::{events::GitHubEvent, graphql::FilesResponse, push::Push, search::Search};
use actix_web::{post, web, HttpRequest, Responder};
use reqwest::Client;
use serde_json::{json, Value};

#[post("/webhook")]
pub async fn webhook(req: HttpRequest, body: web::Json<Value>) -> impl Responder {
    let event_type = GitHubEvent::from_str(
        req.headers()
            .get("X-GitHub-Event")
            .unwrap()
            .to_str()
            .unwrap(),
    )
    .unwrap();

    let payload = body.into_inner();

    if let GitHubEvent::Push = event_type {
        let event: Push = serde_json::from_value(payload).unwrap();
        let repo = &event.repository.full_name;

        let client = Client::builder().use_rustls_tls().build().unwrap();

        let search_url = format!(
            "https://api.github.com/search/code?q=filename:infra.toml+repo:{}",
            repo
        );

        let search_response = client
            .get(&search_url)
            .bearer_auth(std::env::var("GITHUB_TOKEN").unwrap())
            .header("Accept", "application/vnd.github.v3+json")
            .header("User-Agent", "cost-bot")
            .send()
            .await
            .unwrap();

        let search_result = search_response.json::<Search>().await.unwrap();

        let repo_parts: Vec<&str> = repo.split('/').collect();

        let paths: Vec<String> = search_result
            .items
            .iter()
            .map(|item| item.path.clone())
            .collect();

        // Generate the dynamic part of the GraphQL query for each path, with aliases.
        let mut object_queries: Vec<String> = Vec::new();

        for (i, path) in paths.iter().enumerate() {
            let object_query = format!(
                r#"
f{}: object(expression: "HEAD:{}") {{
    ... on Blob {{
        text
    }}
}}"#,
                i, path
            );
            object_queries.push(object_query);
        }

        // Combine all the parts into the full query.
        let query = format!(
            r#"{{
    repository(owner: "{}", name: "{}") {{
        {}
    }}
}}"#,
            repo_parts[0],
            repo_parts[1],
            object_queries.join("\n")
        );

        let response = client
            .post("https://api.github.com/graphql")
            .bearer_auth(std::env::var("GITHUB_TOKEN").unwrap())
            .header("Accept", "application/vnd.github.v3+json")
            .header("User-Agent", "cost-bot")
            .json(&json!({
                "query": query,
            }))
            .send()
            .await
            .unwrap();

        let files = response
            .json::<FilesResponse>()
            .await
            .unwrap()
            .data
            .repository;

        for file in files {
            let _contents = file.1.text;

            // next, parse the contents of the infra.toml file and analyse it.
        }
    }

    "ping"
}

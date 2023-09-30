use std::str::FromStr;

use crate::{
    cost, db, git, github,
    models::{events::GitHubEvent, push::Push},
};
use actix_web::{post, web, HttpRequest, Responder};
use indexmap::IndexMap;
use serde_json::Value;
use sqlx::{Pool, Postgres};
use types::config::InfrastructureConfiguration;

#[post("/webhook")]
pub async fn webhook(
    pool: web::Data<Pool<Postgres>>,

    req: HttpRequest,
    body: web::Json<Value>,
) -> impl Responder {
    let builder = git2::build::RepoBuilder::new();

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

        git::clone(repo.to_string(), String::new(), builder);

        let files = git::configuration_files(repo.to_string());

        git::delete(repo.to_string());

        let mut breakdowns: IndexMap<String, IndexMap<String, f64>> = IndexMap::new();

        for (_path, contents) in files {
            let config = InfrastructureConfiguration::from_str(&contents).unwrap();

            // next, parse the contents of the infra.toml file and analyse it.
            let breakdown = cost::calculate_cost(&config).await;

            breakdowns.insert(config.app.name, breakdown);
        }

        // fetch the previous breakdown
        let previous_breakdown =
            db::fetch_previous_breakdown(&pool, event.repository.id, &event.before)
                .await
                .unwrap();

        // Output the breakdown as a comment on the commit
        github::comment(
            previous_breakdown,
            &breakdowns,
            &event.after,
            &event.repository.full_name,
        )
        .await;

        // Store the breakdown in the database
        db::store_breakdown(&pool, event.repository.id, &event.after, breakdowns)
            .await
            .unwrap();
    }

    "ping"
}

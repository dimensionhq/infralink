use std::{str::FromStr, time::Instant};

use crate::{
    cost, git,
    models::{events::GitHubEvent, push::Push},
};
use actix_web::{post, web, HttpRequest, Responder};
use serde_json::Value;
use types::config::InfrastructureConfiguration;

#[post("/webhook")]
pub async fn webhook(req: HttpRequest, body: web::Json<Value>) -> impl Responder {
    let start = Instant::now();

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

        for (_path, contents) in files {
            let config = InfrastructureConfiguration::from_str(&contents).unwrap();

            // next, parse the contents of the infra.toml file and analyse it.
            cost::calculate_cost(config).await;
        }

        println!("{}", start.elapsed().as_secs_f32());
    }

    "ping"
}

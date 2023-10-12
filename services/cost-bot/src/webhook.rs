use std::str::FromStr;

use crate::{
    cost, db, git, github,
    models::{events::GitHubEvent, pull_request::PullRequest, push::Push},
};
use actix_web::{post, web, HttpRequest, Responder};
use indexmap::IndexMap;
use serde_json::Value;
use sqlx::{Pool, Postgres};
use types::config::InfrastructureConfiguration;

#[post("/webhook")]
pub async fn listener(
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

    match event_type {
        GitHubEvent::Push => {
            let event: Push = serde_json::from_value(payload).unwrap();
            let repo = &event.repository.full_name;

            // clone the repository
            git::clone(
                repo.to_string(),
                String::new(),
                event.after.clone(),
                builder,
            );

            let files = git::configuration_files(repo.to_string());

            // delete the repository once we've detected all the infra.toml files in it
            git::delete(repo.to_string());

            // get a breakdown for each file of the cost and prices
            let breakdowns = git::cost_breakdowns(files).await;

            // fetch the previous breakdown
            let previous_breakdown =
                db::fetch_previous_breakdown(&pool, event.repository.id, &event.before)
                    .await
                    .unwrap();

            // Output the breakdown as a comment on the commit
            let id = github::comment_on_commit(
                previous_breakdown,
                &breakdowns,
                &event.after,
                &event.repository.full_name,
            )
            .await;

            // Mark the check run for the commit as successful
            // github::mark_check_run(
            //     &event.repository.full_name,
            //     &event.after,
            //     "success",
            //     "Cost Analysis",
            //     "Cost breakdown",
            //     id,
            // )
            // .await;

            // Store the breakdown in the database
            db::store_breakdown(&pool, event.repository.id, &event.after, breakdowns)
                .await
                .unwrap();
        }
        GitHubEvent::PullRequest => {
            let event: PullRequest = serde_json::from_value(payload).unwrap();

            match event.action.as_str() {
                "opened" | "synchronize" => {
                    let repo = &event.repository.full_name;

                    git::clone(
                        repo.to_string(),
                        String::new(),
                        event.pull_request.head.sha.clone(),
                        builder,
                    );

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
                    let previous_breakdown = db::fetch_previous_breakdown(
                        &pool,
                        event.repository.id,
                        &event.pull_request.base.sha,
                    )
                    .await
                    .unwrap();

                    github::comment_on_pull_request(
                        previous_breakdown,
                        &breakdowns,
                        event.number,
                        &event.repository.full_name,
                    )
                    .await;

                    // todo: mark checks on the pull request
                    

                    // this is based on the cost guidelines
                }
                &_ => {}
            }
        }
        _ => {}
    }

    "ping"
}

use serde::Deserialize;

use super::push::Repository;

#[derive(Deserialize, Debug)]
pub struct PullRequest {
    pub action: String,
    pub number: u64,
    pub pull_request: PullRequestData,
    pub repository: Repository,
}

#[derive(Deserialize, Debug)]
pub struct PullRequestData {
    pub head: Head,
    pub base: Base,
    pub merged: bool,
    pub merged_commit_sha: Option<String>,
}

#[derive(Deserialize, Debug)]
pub struct Head {
    pub r#ref: String,
    pub sha: String,
}

#[derive(Deserialize, Debug)]
pub struct Base {
    pub r#ref: String,
    pub sha: String,
}

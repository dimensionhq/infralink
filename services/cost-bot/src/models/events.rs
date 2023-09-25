use std::str::FromStr;

pub enum GitHubEvent {
    Push,
    Ping,
}

impl FromStr for GitHubEvent {
    type Err = ();

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s {
            "push" => Ok(GitHubEvent::Push),
            "ping" => Ok(GitHubEvent::Ping),
            _ => Err(()),
        }
    }
}

use serde::Deserialize;

#[derive(Debug, Deserialize)]
pub struct Push {
    // ref after the push
    pub after: String,

    // ref before the push
    pub before: String,

    // primary commit
    pub head_commit: Commit,

    // repository
    pub repository: Repository,
}

#[derive(Debug, Deserialize)]
pub struct Commit {
    // id of the commit
    pub id: String,

    // added files
    pub added: Vec<String>,

    // removed files
    pub removed: Vec<String>,

    // modified files
    pub modified: Vec<String>,

    // commit message
    pub message: String,

    // author
    pub author: Author,
}

#[derive(Debug, Deserialize)]
pub struct Author {
    // name of the author
    pub name: String,

    // username of the author
    pub username: String,

    // email of the author
    pub email: String,
}

#[derive(Debug, Deserialize)]
pub struct Repository {
    // repository id
    pub id: i64,

    // full name of the repository
    pub full_name: String,
}

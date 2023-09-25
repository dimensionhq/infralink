use std::collections::HashMap;

use serde::Deserialize;

#[derive(Deserialize, Debug)]
pub struct FilesResponse {
    pub data: Data,
}

#[derive(Deserialize, Debug)]
pub struct Data {
    pub repository: HashMap<String, File>,
}

#[derive(Deserialize, Debug)]
pub struct File {
    pub text: String,
}

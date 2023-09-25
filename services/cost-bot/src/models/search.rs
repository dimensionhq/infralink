use serde::Deserialize;

#[derive(Debug, Deserialize)]
pub struct Search {
    pub total_count: i64,
    pub items: Vec<SearchItem>,
}

#[derive(Debug, Deserialize)]
pub struct SearchItem {
    pub path: String,
}

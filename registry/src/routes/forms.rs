use serde::{Deserialize, Serialize};

#[derive(Debug, Deserialize)]
pub struct SearchForm {
    pub search_terms: Vec<String>,
}

#[derive(Debug, Serialize)]
pub struct SearchResult {
    pub name: String,
    pub description: String,
}

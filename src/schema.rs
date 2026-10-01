use serde::Deserialize;

#[derive(Debug, Deserialize, Default)]
pub struct FilterOptions {
    pub page: Option<u32>,
    pub limit: Option<u32>,
}

#[derive(Debug, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct CreateNoteSchema {
    pub title: String,
    pub content: String,
    pub category: Option<String>,
    pub published: Option<bool>,
}

#[derive(Debug, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct UpdateNoteSchema {
    pub title: Option<String>,
    pub content: Option<String>,
    pub category: Option<String>,
    pub published: Option<bool>,
}

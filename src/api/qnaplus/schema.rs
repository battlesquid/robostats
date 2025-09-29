use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Clone, Default, Debug, PartialEq)]
#[serde(rename_all = "camelCase")]
pub struct PartialQuestion {
    pub id: String,
    pub program: String,
    pub season: String,
    pub url: String,
    pub title: String,
    pub author: String,
    pub answered: bool,
    pub asked_timestamp: String,
    pub asked_timestamp_ms: i64,
    pub answered_timestamp: Option<String>,
    pub answered_timestamp_ms: Option<i64>,
    pub tags: Vec<String>
}
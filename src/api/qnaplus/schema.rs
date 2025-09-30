use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Clone, Default, Debug)]
#[serde(rename_all = "camelCase")]
pub struct PartialQuestion {
    pub id: String,
    pub program: String,
    pub season: String,
    pub url: String,
    pub title: String,
    pub author: String,
    pub answered: bool,
    pub asked_timestamp_ms: i64,
    pub answered_timestamp_ms: Option<i64>,
    pub tags: Vec<String>,
}

#[derive(Serialize, Deserialize, Clone, Default, Debug)]
pub struct Rule {
    pub rule: String,
    pub description: String,
    pub link: String,
    pub questions: Vec<PartialQuestion>,
}

#[derive(Serialize, Deserialize, Clone, Default, Debug)]
pub struct RuleError {
    pub message: String,
    pub error: Option<String>,
}

#[derive(Serialize, Deserialize, Debug)]
#[serde(untagged)]
pub enum RuleResponse {
    RuleError(RuleError),
    Rule(Rule),
}

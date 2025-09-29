use reqwest::header::USER_AGENT;
use std::time::Duration;

use crate::api::qnaplus::schema::PartialQuestion;

#[derive(Default, Debug, Clone)]
pub struct Qnaplus {
    pub req_client: reqwest::Client,
}

pub const API_BASE: &str = "https://api.qnapl.us/api";

impl Qnaplus {
    pub fn new() -> Self {
        Self {
            req_client: reqwest::Client::new(),
        }
    }
    async fn request(
        &self,
        endpoint: impl AsRef<str>,
    ) -> Result<reqwest::Response, reqwest::Error> {
        Ok(self
            .req_client
            .get(format!("{API_BASE}{}", endpoint.as_ref()))
            .header("accept-language", "en")
            .header(USER_AGENT, "RoboStats Discord Bot")
            .timeout(Duration::from_secs(10))
            .send()
            .await?)
    }

    pub async fn get_qnas_for_rule(&self, rule_name: &str, season: Option<&str>) -> Result<PartialQuestion, reqwest::Error> {
        let response = self
            .request(format!("/rules/{}/qnas?season={}", rule_name, season.unwrap_or("")))
            .await?;
        Ok(response.json().await?)
    }
}

use crate::envs;
use crate::redmine::model::{RedmineIssue, RedmineIssueNotes};
use typed_builder::TypedBuilder;

#[derive(TypedBuilder)]
pub struct RedmineAction {
    url: String,
    api_key: String,
}

#[derive(thiserror::Error, Debug)]
pub enum RedmineActionError {
    #[error("Failed to send request to Redmine.")]
    FailedToSendRequest,
    #[error("{0}")]
    FailedToPostComment(String),
}

impl RedmineAction {
    pub fn new() -> Self {
        let envs = envs();
        RedmineAction::builder()
            .url(envs.redmine_url.clone())
            .api_key(envs.redmine_api_key.clone())
            .build()
    }

    pub fn issue_url(&self, id: u16) -> String {
        format!("{}/issues/{}.json?key={}", self.url, id, self.api_key)
    }

    pub async fn run(id: u16, content: String) -> anyhow::Result<(), RedmineActionError> {
        let client = reqwest::Client::new();

        let url = RedmineAction::new().issue_url(id);
        let body = RedmineIssue::builder()
            .issue(RedmineIssueNotes::builder().notes(content).build())
            .build();

        let res = client
            .put(&url)
            .json(&body)
            .send()
            .await
            .map_err(|_| RedmineActionError::FailedToSendRequest)?;

        if res.status().is_success() {
            Ok(())
        } else {
            Err(RedmineActionError::FailedToPostComment(
                res.text().await.unwrap_or_default(),
            ))
        }
    }
}

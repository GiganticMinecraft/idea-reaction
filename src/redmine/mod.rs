/// ref. https://github.com/GiganticMinecraft/IdeaDiscussionMaster/blob/main/c-infra/src/redmine/client/redmine_url_interpreter.rs
use crate::envs;
use crate::redmine::model::{RedmineIssue, RedmineIssueNotes};
use anyhow::{anyhow, Context, Error};
use typed_builder::TypedBuilder;

pub mod model;

#[derive(TypedBuilder)]
pub struct RedmineAction {
    url: String,
    api_key: String,
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

    pub async fn run(id: u16, content: String) -> anyhow::Result<(), Error> {
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
            .context("Failed to send request")?;

        if res.status().is_success() {
            Ok(())
        } else {
            Err(anyhow!(
                "Failed to post comment. Reason: {}",
                res.text().await?
            ))
        }
    }
}

use typed_builder::TypedBuilder;

#[derive(serde::Serialize, serde::Deserialize, Debug, TypedBuilder)]
pub struct RedmineIssue {
    pub issue: RedmineIssueNotes,
}

#[derive(serde::Serialize, serde::Deserialize, Debug, TypedBuilder)]
pub struct RedmineIssueNotes {
    pub notes: String,
}

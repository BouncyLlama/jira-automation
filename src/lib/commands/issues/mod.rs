mod list_transition;
mod search_issues;
mod transition_issue;
mod update;
pub use list_transition::*;
pub use search_issues::*;
use serde::{Deserialize, Serialize};
pub use transition_issue::*;
pub use update::*;

const ISSUE_NAME_HELP: &str = "the ticket name ex. FOO-1234";
const FIX_VERSION_HELP: &str = "the version which resolves this issue";

const RELATED_VERSION_HELP: &str = "the version this issue is related to";

const TRANSITION_HELP: &str = "the name or id of transition to set";
const INCLUDE_UNAVAILABLE_HELP: &str =
    "include transitions which are not currently possible for this issue";
const BY_ID_HELP: &str = "perform the operation by specifying an ID rather than a name";

#[derive(Serialize, Deserialize, Debug, Clone)]
#[serde(rename_all = "camelCase")]
pub struct Transition {
    id: String,
    name: String,
    is_available: bool,
}

#[derive(Serialize, Deserialize, Debug)]
#[serde(rename_all = "camelCase")]
pub struct IssueStatus {
    description: Option<String>,
    id: String,
    name: String,
}

#[derive(Serialize, Deserialize, Debug)]
#[serde(rename_all = "camelCase")]
pub struct IssueFields {
    summary: String,
    status: IssueStatus,
    fix_versions: Vec<IssueRelease>,
}

#[derive(Serialize, Deserialize, Debug)]
#[serde(rename_all = "camelCase")]
pub struct IssueRelease {
    id: String,
    description: Option<String>,
    name: String,
}
#[derive(Serialize, Deserialize, Debug)]
#[serde(rename_all = "camelCase")]
pub struct Issue {
    id: String,
    pub(crate) key: String,
    fields: IssueFields,
}

#[derive(Serialize, Deserialize, Debug)]
#[serde(rename_all = "camelCase")]
pub struct PaginatedIssues {
    pub(crate) total: u64,
    pub(crate) start_at: u64,
    pub(crate) issues: Vec<Issue>,
}

mod transition_issue;
mod list_transition;
mod update;
use serde::{Deserialize, Serialize};
pub use list_transition::*;
use update::*;
pub use transition_issue::*;
pub use update::*;
const issueNameHelp: &str  = "the ticket name ex. FOO-1234";
const fixVersionHelp: &str  = "the version which resolves this issue";

const relatedVersionHelp: &str  = "the version this issue is related to";

const transitionHelp: &str  = "the name or id of transition to set";
const includeUnavailableHelp: &str="include transitions which are not currently possible for this issue";
const byIdHelp: &str="perform the operation by specifying an ID rather than a name";

#[derive(Serialize,Deserialize,Debug, Clone)]
pub struct Transition{
    id: String,
    name:String,
    isAvailable: bool
}



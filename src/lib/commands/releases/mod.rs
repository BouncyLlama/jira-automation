

use clap::arg;


use log::{debug};
use serde::Deserialize;
use serde::Serialize;

pub use create::*;
pub use delete::*;
pub use list::*;
pub use update::*;

use crate::Cli;


mod create;
mod delete;
mod list;
mod update;

const PROJECT_HELP: &str = "project identifier";
const NAME_HELP: &str = "name of the release";
const DESCRIPTION_HELP: &str = "description of the release";
const START_DATE_HELP: &str = "start date of the version in ISO 8601 format (yyyy-mm-dd)";
const RELEASE_DATE_HELP: &str = "release date of the version in ISO 8601 format (yyyy-mm-dd)";
const BY_ID_HELP: &str =
    "perform operation by specifying id rather than name (useful if your names are not unique)";
const RELEASE_HELP: &str = "the name or id of the release to perform the operation upon";

#[derive(Serialize, Deserialize, Debug)]
#[serde(rename_all = "camelCase")]

pub struct Release {
    pub(crate) id: String,
    pub(crate) description: Option<String>,
    pub(crate) name: Option<String>,
    pub(crate) archived: bool,
    pub(crate) released: bool,
    pub(crate) release_date: Option<String>,
    pub(crate) overdue: Option<bool>,
    pub(crate) user_release_date: Option<String>,
    pub(crate) project_id: u64,
}

#[derive(Serialize, Deserialize, Debug)]
#[serde(rename_all = "camelCase")]

pub struct PaginatedReleases<T> {
    pub(crate) total: u64,
    pub(crate) start_at: u64,
    pub(crate) is_last: bool,
    pub(crate) values: Vec<T>,
}

fn get_id_from_name(
    ctx: &Cli,
    project: String,
    name: String,
) -> Result<String, Box<dyn std::error::Error>> {
    let args = ListReleasesArgs {
        project,
        filter: Option::from(name),
        unpaginate: true,
        page_size: 100,
        page_start_idx: 0,
    };
    let  result = do_list_releases(ctx, &args)?;
    if result.len() != 1 {
        Err(Box::try_from("release name matches multiple releases").unwrap())
    } else {
        let id = result.get(0).unwrap().id.clone();
        Ok(id)
    }
}

use std::fmt::Write;
use super::super::*;
use clap::Parser;
use log::error;
use clap::Subcommand;
use clap::ArgGroup;
use log::info;
use log::warn;
use log::trace;
use log::{debug, LevelFilter};
use clap::arg;
use crate::lib::util;
use serde::Serialize;
use serde::Deserialize;
use std::{io, thread};
use std::borrow::Borrow;
use std::collections::HashMap;
use std::io::BufRead;
use std::time::Duration;
use crate::Cli;

mod list;
mod create;
mod delete;
mod update;

pub use create::*;
pub use list::*;
pub use delete::*;
pub use update::*;

const projectHelp: &str = "project identifier";
const nameHelp: &str = "name of the release";
const descriptionHelp: &str = "description of the release";
const startDateHelp: &str = "start date of the version in ISO 8601 format (yyyy-mm-dd)";
const releaseDateHelp: &str = "release date of the version in ISO 8601 format (yyyy-mm-dd)";
const byIdHelp: &str = "perform operation by specifying id rather than name (useful if your names are not unique)";
const releaseHelp: &str = "the name or id of the release to perform the operation upon";


#[derive(Serialize, Deserialize, Debug)]
pub struct Release {
    id: String,
    description: Option<String>,
    name: Option<String>,
    archived: bool,
    released: bool,
    releaseDate: Option<String>,
    overdue: Option<bool>,
    userReleaseDate: Option<String>,
    projectId: u64,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct PaginatedReleases<T> {
    pub(crate) total: u64,
    pub(crate) startAt: u64,
    pub(crate) isLast: bool,
    pub(crate) values: Vec<T>,
}


fn get_id_from_name(ctx: &Cli, project: String, name: String) -> Result<String, Box<dyn std::error::Error>> {
    let args = ListReleasesArgs {
        project,
        filter: Option::from(name),
        unpaginate: true,
        page_size: 100,
        page_start_idx: 0,
    };
    let mut result = do_list_releases(ctx, &args)?;
    if result.len() != 1 {
        Err(Box::try_from("release name matches multiple releases").unwrap())
    } else {
        let id = result.get(0).unwrap().id.clone();
        Ok(id)
    }
}












use std::collections::HashMap;
use crate::Cli;
use crate::lib::commands::releases::{PaginatedReleases, Release};
use crate::lib::util;
use clap::Parser;
use super::*;

#[derive(Parser, Clone)]
#[command()]
pub struct CreateReleaseArgs {
    #[arg(long, short, help = projectHelp)]
    pub(crate) project: String,
    #[arg(long, short, help = nameHelp)]
    pub(crate) name: String,
    #[arg(long, short, help = descriptionHelp)]
    pub(crate) description: Option<String>,
    #[arg(long, short, help = startDateHelp)]
    pub(crate) startDate: Option<String>,
    #[arg(long, short, help = releaseDateHelp)]
    pub(crate) releaseDate: Option<String>,
}

fn assemble_create_args(args: CreateReleaseArgs) -> HashMap<&'static str, String> {
    let mut params: HashMap<&str, String> = HashMap::new();
    params.insert("name", args.name);
    params.insert("project", args.project);
    if args.description.is_some() {
        params.insert("description", args.description.unwrap());
    }
    if args.startDate.is_some() {
        params.insert("startDate", args.startDate.unwrap());
    }
    if args.releaseDate.is_some() {
        params.insert("releaseDate", args.releaseDate.unwrap());
    }
    return params;
}

pub fn execute_create_release(ctx: &Cli, args: &CreateReleaseArgs) -> Result<(), Box<dyn std::error::Error>> {
    let reqUrl = format!("{}/rest/api/3/version", ctx.baseJiraUrl, );
    let result = util::doPost::<Release, HashMap<&str, String>>(&reqUrl, ctx, &(assemble_create_args(args.clone())))?;
    Ok(())
}

use std::collections::HashMap;
use crate::Cli;
use crate::lib::commands::releases::{ Release};
use crate::lib::util;
use clap::Parser;
use super::*;

#[derive(Parser, Clone)]
#[command()]
pub struct CreateReleaseArgs {
    #[arg(long, short, help = PROJECT_HELP)]
    pub(crate) project: String,
    #[arg(long, short, help = NAME_HELP)]
    pub(crate) name: String,
    #[arg(long, short, help = DESCRIPTION_HELP)]
    pub(crate) description: Option<String>,
    #[arg(long, short, help = START_DATE_HELP)]
    pub(crate) start_date: Option<String>,
    #[arg(long, short, help = RELEASE_DATE_HELP)]
    pub(crate) release_date: Option<String>,
}

fn assemble_create_args(args: CreateReleaseArgs) -> HashMap<&'static str, String> {
    let mut params: HashMap<&str, String> = HashMap::new();
    params.insert("name", args.name);
    params.insert("project", args.project);
    if args.description.is_some() {
        params.insert("description", args.description.unwrap());
    }
    if args.start_date.is_some() {
        params.insert("startDate", args.start_date.unwrap());
    }
    if args.release_date.is_some() {
        params.insert("releaseDate", args.release_date.unwrap());
    }
    params
}

pub fn execute_create_release(ctx: &Cli, args: &CreateReleaseArgs) -> Result<(), Box<dyn std::error::Error>> {
    let req_url = format!("{}/rest/api/3/version", ctx.base_jira_url, );
    let result = util::do_post::<Release, HashMap<&str, String>>(&req_url, ctx, &(assemble_create_args(args.clone())))?;
    if result.is_some() {
        util::format_print::<Release>(Vec::from([result.unwrap()]), ctx.output_format)?;
    }

    Ok(())
}

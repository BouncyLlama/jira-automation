use super::*;
use crate::lib::{util, AppError};
use crate::Cli;
use clap::Parser;
use std::collections::HashMap;

#[derive(Parser, Clone)]
#[command()]
pub struct UpdateReleaseArgs {
    #[arg(long, short, help = NAME_HELP)]
    pub(crate) name: Option<String>,
    #[arg(long, short, help = DESCRIPTION_HELP)]
    pub(crate) description: Option<String>,
    #[arg(long, help = START_DATE_HELP)]
    pub(crate) start_date: Option<String>,
    #[arg(long, help = RELEASE_DATE_HELP)]
    pub(crate) release_date: Option<String>,
    #[arg(long, short, help = "whether or not this release has been 'released'")]
    pub(crate) is_released: Option<bool>,
    #[arg(long, short, help = BY_ID_HELP)]
    pub(crate) by_id: Option<bool>,
    #[arg(long, short, help = "the name (or id) of the release to update")]
    pub(crate) release: String,
    #[arg(long, short, help = PROJECT_HELP)]
    pub(crate) project: String,
}

impl UpdateReleaseArgs {
    fn set_release(&mut self, new: String) {
        self.release = new;
    }
}

pub fn execute_update_release(ctx: &Cli, args: &UpdateReleaseArgs) -> Result<(), AppError> {
    do_update_release(ctx, args)
}

pub fn do_update_release(ctx: &Cli, args: &UpdateReleaseArgs) -> Result<(), AppError> {
    let mut mutargs = args.clone();
    if args.by_id.is_some() && args.by_id.unwrap() == true {
        let req_url = format!("{}/rest/api/3/version/{}", ctx.base_jira_url, args.release);

        util::do_put::<(), HashMap<&str, String>>(
            &req_url,
            ctx,
            &(assemble_update_args(args.clone())),
        )?;
    } else {
        let id = get_id_from_name(ctx, args.project.clone(), args.release.clone())?;
        let req_url = format!("{}/rest/api/3/version/{}", ctx.base_jira_url, id);

        debug!("found release {} for name {}", id, args.release);
        mutargs.set_release(id);
        util::do_put::<(), HashMap<&str, String>>(
            &req_url,
            ctx,
            &(assemble_update_args(mutargs.clone())),
        )?;
    }

    Ok(())
}

fn assemble_update_args(args: UpdateReleaseArgs) -> HashMap<&'static str, String> {
    let mut params: HashMap<&str, String> = HashMap::new();
    if args.start_date.is_some() {
        params.insert("startDate", args.start_date.unwrap());
    }
    if args.release_date.is_some() {
        params.insert("releaseDate", args.release_date.unwrap());
    }
    if args.description.is_some() {
        params.insert("description", args.description.unwrap());
    }
    if args.name.is_some() {
        params.insert("name", args.name.unwrap());
    }
    if args.is_released.is_some() {
        params.insert("released", args.is_released.unwrap().to_string());
    }

    return params;
}

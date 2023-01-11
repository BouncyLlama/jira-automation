use std::collections::HashMap;
use crate::Cli;
use crate::lib::commands::releases::{PaginatedReleases, Release};
use crate::lib::util;
use clap::Parser;
use super::*;


#[derive(Parser, Clone)]
#[command()]
pub struct UpdateReleaseArgs {
    #[arg(long, short, help = nameHelp)]
    pub(crate) name: Option<String>,
    #[arg(long, short, help = descriptionHelp)]
    pub(crate) description: Option<String>,
    #[arg(long, help = startDateHelp)]
    pub(crate) startDate: Option<String>,
    #[arg(long, help = releaseDateHelp)]
    pub(crate) releaseDate: Option<String>,
    #[arg(long, short, help = "whether or not this release has been 'released'")]
    pub(crate) is_released: Option<bool>,
    #[arg(long, short, help = byIdHelp)]
    pub(crate) byId: Option<bool>,
    #[arg(long, short, help = "the name (or id) of the release to update")]
    pub(crate) release: String,
    #[arg(long, short, help = projectHelp)]
    pub(crate) project: String,
}


impl UpdateReleaseArgs {
    fn set_release(&mut self, new: String) {
        self.release = new;
    }
}

pub fn execute_update_release(ctx: &Cli, args: &UpdateReleaseArgs) -> Result<(), Box<dyn std::error::Error>> {
    let mut mutargs = args.clone();
    if args.byId.is_some() && args.byId.unwrap() == true {
        let reqUrl = format!("{}/rest/api/3/version/{}", ctx.baseJiraUrl, args.release);

        util::doPut::<(), HashMap<&str, String>>(&reqUrl, ctx, &(assemble_update_args(args.clone())))?;
    } else {
        let id = get_id_from_name(ctx, args.project.clone(), args.release.clone())?;
        let reqUrl = format!("{}/rest/api/3/version/{}", ctx.baseJiraUrl, id);

        debug!("found release {} for name {}",id,args.release);
        mutargs.set_release(id);
        util::doPut::<(), HashMap<&str, String>>(&reqUrl, ctx, &(assemble_update_args(mutargs.clone())))?;
    }

    Ok(())
}

fn assemble_update_args(args: UpdateReleaseArgs) -> HashMap<&'static str, String> {
    let mut params: HashMap<&str, String> = HashMap::new();
    if args.startDate.is_some() {
        params.insert("startDate", args.startDate.unwrap());
    }
    if args.releaseDate.is_some() {
        params.insert("releaseDate", args.releaseDate.unwrap());
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
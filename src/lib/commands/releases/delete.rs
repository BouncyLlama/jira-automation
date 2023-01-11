use std::collections::HashMap;
use crate::Cli;
use crate::lib::commands::releases::{PaginatedReleases, Release};
use crate::lib::util;
use clap::Parser;
use super::*;

#[derive(Parser, Clone)]
#[command()]
pub struct DeleteReleaseArgs {
    #[arg(long, short, help = releaseHelp)]
    pub(crate) release: String,
    #[arg(long, short, help = projectHelp)]
    pub(crate) project: String,
    #[arg(long, short, help = byIdHelp)]
    pub(crate) byId: Option<bool>,
    #[arg(long, help = "for tickets referencing this release, replace fix version with this other release")]
    pub(crate) replaceFixVersion: Option<String>,
    #[arg(long, help = "for tickets referencing this release, replace affected version with this other release")]
    pub(crate) replaceAffectedVersion: Option<String>,

}

impl DeleteReleaseArgs {
    fn set_release(&mut self, new: String) {
        self.release = new;
    }
    fn set_fixversion(&mut self, new: String) {
        self.replaceFixVersion = Option::from(new);
    }
    fn set_affectedversion(&mut self, new: String) {
        self.replaceAffectedVersion = Option::from(new);
    }
}

pub fn execute_delete_release(ctx: &Cli, args: &DeleteReleaseArgs) -> Result<(), Box<dyn std::error::Error>> {
    let mut mutargs = args.clone();
    if args.byId.is_some() && args.byId.unwrap() == true {
        let reqUrl = format!("{}/rest/api/3/version/{}/removeAndSwap", ctx.baseJiraUrl, args.release);

        util::doPost::<(), HashMap<&str, String>>(&reqUrl, ctx, &(assemble_delete_args(args.clone())))?;
    } else {
        let id = get_id_from_name(ctx, args.project.clone(), args.release.clone())?;
        let reqUrl = format!("{}/rest/api/3/version/{}/removeAndSwap", ctx.baseJiraUrl, id);

        debug!("found release {} for name {}",id,args.release);
        mutargs.set_release(id);
        if args.replaceFixVersion.is_some() {
            let fixid = get_id_from_name(ctx, args.project.clone(), args.replaceFixVersion.as_ref().unwrap().clone())?;
            mutargs.set_fixversion(fixid);
        }
        if args.replaceAffectedVersion.is_some() {
            let affectedid = get_id_from_name(ctx, args.project.clone(), args.replaceAffectedVersion.as_ref().unwrap().clone())?;
            mutargs.set_fixversion(affectedid);
        }
        util::doPost::<(), HashMap<&str, String>>(&reqUrl, ctx, &(assemble_delete_args(mutargs.clone())))?;
    }

    Ok(())
}

fn assemble_delete_args(args: DeleteReleaseArgs) -> HashMap<&'static str, String> {
    let mut params: HashMap<&str, String> = HashMap::new();
    if args.replaceAffectedVersion.is_some() {
        params.insert("moveAffectedIssuesTo", args.replaceAffectedVersion.unwrap());
    }
    if args.replaceFixVersion.is_some() {
        params.insert("moveFixIssuesTo", args.replaceFixVersion.unwrap());
    }

    return params;
}
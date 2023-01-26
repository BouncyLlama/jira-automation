use super::*;
use crate::lib::util;
use crate::Cli;
use clap::Parser;
use std::collections::HashMap;

#[derive(Parser, Clone)]
#[command()]
pub struct DeleteReleaseArgs {
    #[arg(long, short, help = RELEASE_HELP)]
    pub(crate) release: String,
    #[arg(long, short, help = PROJECT_HELP)]
    pub(crate) project: String,
    #[arg(long, short, help = BY_ID_HELP)]
    pub(crate) by_id: Option<bool>,
    #[arg(
        long,
        help = "for tickets referencing this release, replace fix version with this other release"
    )]
    pub(crate) replace_fix_version: Option<String>,
    #[arg(
        long,
        help = "for tickets referencing this release, replace affected version with this other release"
    )]
    pub(crate) replace_affected_version: Option<String>,
}

impl DeleteReleaseArgs {
    fn set_release(&mut self, new: String) {
        self.release = new;
    }
    fn set_fixversion(&mut self, new: String) {
        self.replace_fix_version = Option::from(new);
    }
    fn set_affectedversion(&mut self, new: String) {
        self.replace_affected_version = Option::from(new);
    }
}

pub fn execute_delete_release(ctx: &Cli, args: &DeleteReleaseArgs) -> Result<(), AppError> {
    let mut mutargs = args.clone();
    if args.by_id.is_some() && args.by_id.unwrap() == true {
        let req_url = format!(
            "{}/rest/api/3/version/{}/removeAndSwap",
            ctx.base_jira_url, args.release
        );

        util::do_post::<(), HashMap<&str, String>>(
            &req_url,
            ctx,
            &(assemble_delete_args(args.clone())),
        )?;
    } else {
        let id = get_id_from_name(ctx, args.project.clone(), args.release.clone())?;
        let req_url = format!(
            "{}/rest/api/3/version/{}/removeAndSwap",
            ctx.base_jira_url, id
        );

        debug!("found release {} for name {}", id, args.release);
        mutargs.set_release(id);
        if args.replace_fix_version.is_some() {
            let fixid = get_id_from_name(
                ctx,
                args.project.clone(),
                args.replace_fix_version.as_ref().unwrap().clone(),
            )?;
            mutargs.set_fixversion(fixid);
        }
        if args.replace_affected_version.is_some() {
            let affectedid = get_id_from_name(
                ctx,
                args.project.clone(),
                args.replace_affected_version.as_ref().unwrap().clone(),
            )?;
            mutargs.set_affectedversion(affectedid);
        }
        util::do_post::<(), HashMap<&str, String>>(
            &req_url,
            ctx,
            &(assemble_delete_args(mutargs.clone())),
        )?;
    }

    Ok(())
}

fn assemble_delete_args(args: DeleteReleaseArgs) -> HashMap<&'static str, String> {
    let mut params: HashMap<&str, String> = HashMap::new();
    if args.replace_affected_version.is_some() {
        params.insert(
            "moveAffectedIssuesTo",
            args.replace_affected_version.unwrap(),
        );
    }
    if args.replace_fix_version.is_some() {
        params.insert("moveFixIssuesTo", args.replace_fix_version.unwrap());
    }

    return params;
}

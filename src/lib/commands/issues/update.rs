use super::*;
use crate::lib::{util, AppError};
use crate::Cli;
use clap::ArgGroup;
use clap::Parser;
use std::collections::HashMap;

#[derive(Parser, Clone)]
#[command(group(ArgGroup::new("vers").required(true).args(["related_version", "fix_version"]),))]
pub struct UpdateIssueArgs {
    #[arg(long, short, help = ISSUE_NAME_HELP)]
    pub(crate) name: String,
    #[arg(long, short, help = FIX_VERSION_HELP)]
    pub(crate) fix_version: Option<String>,
    #[arg(long, short, help = RELATED_VERSION_HELP)]
    pub(crate) related_version: Option<String>,
    #[arg(long, short, help = BY_ID_HELP, default_value_t = false)]
    pub(crate) use_version_id: bool,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct UpdateIssueRequest {
    update: HashMap<String, Vec<HashMap<String, HashMap<String, String>>>>,
}

pub fn execute_update_issue(ctx: &Cli, args: &UpdateIssueArgs) -> Result<(), AppError> {
    do_update(ctx, args)
}

fn do_update(ctx: &Cli, args: &UpdateIssueArgs) -> Result<(), AppError> {
    let mut req = UpdateIssueRequest {
        update: HashMap::new(),
    };
    if args.fix_version.is_some() {
        if args.use_version_id {
            req.update.insert(
                "fixVersions".parse().unwrap(),
                Vec::from([HashMap::from([(
                    "add".parse().unwrap(),
                    HashMap::from([("id".parse().unwrap(), args.fix_version.clone().unwrap())]),
                )])]),
            );
        } else {
            req.update.insert(
                "fixVersions".parse().unwrap(),
                Vec::from([HashMap::from([(
                    "add".parse().unwrap(),
                    HashMap::from([("name".parse().unwrap(), args.fix_version.clone().unwrap())]),
                )])]),
            );
        }
    }
    if args.related_version.is_some() {
        if args.use_version_id {
            req.update.insert(
                "relatedVersions".parse().unwrap(),
                Vec::from([HashMap::from([(
                    "add".parse().unwrap(),
                    HashMap::from([("id".parse().unwrap(), args.related_version.clone().unwrap())]),
                )])]),
            );
        } else {
            req.update.insert(
                "relatedVersions".parse().unwrap(),
                Vec::from([HashMap::from([(
                    "add".parse().unwrap(),
                    HashMap::from([(
                        "name".parse().unwrap(),
                        args.related_version.clone().unwrap(),
                    )]),
                )])]),
            );
        }
    }

    let req_url = format!("{}/rest/api/3/issue/{}", ctx.base_jira_url, args.name);
    util::do_put::<(), UpdateIssueRequest>(&req_url, ctx, &req)?;
    Ok(())
}

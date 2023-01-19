use std::collections::HashMap;
use std::error::Error;
use std::iter::Map;
use crate::Cli;
use crate::lib::commands::issues::Transition;
use crate::lib::util;
use clap::Parser;
use clap::ArgGroup;
use super::*;

#[derive(Parser, Clone)]
#[command(group(ArgGroup::new("vers").required(true).args(["relatedVersion", "fixVersion"]),))]
pub struct UpdateIssueArgs {
    #[arg(long, short, help = issueNameHelp)]
    pub(crate) name: String,
    #[arg(long, short, help = fixVersionHelp)]
    pub(crate) fixVersion: Option<String>,
    #[arg(long, short, help = relatedVersionHelp)]
    pub(crate) relatedVersion: Option<String>,
    #[arg(long, short, help = byIdHelp, default_value_t = false)]
    pub(crate) useVersionId: bool,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct UpdateIssueRequest {
    update: HashMap<String, Vec<HashMap<String, HashMap<String, String>>>>,
}

pub fn execute_update_issue(ctx: &Cli, args: &UpdateIssueArgs) -> Result<(), Box<dyn Error>> {
    do_update(ctx, args)
}

fn do_update(ctx: &Cli, args: &UpdateIssueArgs) -> Result<(), Box<dyn Error>> {
    let mut req = UpdateIssueRequest {
        update: HashMap::new()
    };
    if args.fixVersion.is_some() {
        if args.useVersionId {
            req.update.insert("fixVersions".parse().unwrap(), Vec::from([HashMap::from([("add".parse().unwrap(), HashMap::from([("id".parse().unwrap(), args.fixVersion.clone().unwrap())]))])]));
        } else {
            req.update.insert("fixVersions".parse().unwrap(), Vec::from([HashMap::from([("add".parse().unwrap(), HashMap::from([("name".parse().unwrap(), args.fixVersion.clone().unwrap())]))])]));
        }
    }
    if args.relatedVersion.is_some() {
        if args.useVersionId {
            req.update.insert("relatedVersions".parse().unwrap(), Vec::from([HashMap::from([("add".parse().unwrap(), HashMap::from([("id".parse().unwrap(), args.relatedVersion.clone().unwrap())]))])]));
        } else {
            req.update.insert("relatedVersions".parse().unwrap(), Vec::from([HashMap::from([("add".parse().unwrap(), HashMap::from([("name".parse().unwrap(), args.relatedVersion.clone().unwrap())]))])]));
        }
    }

    let reqUrl = format!("{}/rest/api/3/issue/{}", ctx.baseJiraUrl, args.name);
    let result = util::doPut::<(), UpdateIssueRequest>(&reqUrl, ctx, &req)?;
    Ok(())
}

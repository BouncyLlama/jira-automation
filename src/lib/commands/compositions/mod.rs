use super::*;
use crate::lib::commands::issues::{
    do_search_issues, execute_update_issue, SearchIssuesArgs, UpdateIssueArgs,
};
use crate::lib::commands::releases::{
    do_update_release, CreateReleaseArgs, UpdateReleaseArgs,
};
use crate::lib::AppError;
use crate::lib::AppError::CouldNotCreateRelease;
use crate::Cli;
use chrono::{DateTime, Utc};
use clap::Parser;
use log::warn;
use releases::DESCRIPTION_HELP;
use releases::NAME_HELP;
use releases::PROJECT_HELP;

#[derive(Parser, Clone)]
#[command()]
pub struct ReleaseCompletedIssuesArgs {
    #[arg(long, short, help = NAME_HELP)]
    pub release_name: String,

    #[arg(long, short, help = PROJECT_HELP)]
    pub project: String,

    #[arg(long, short, help = "JQL query to determine which issues to release")]
    pub issue_jql: Option<String>,

    #[arg(long, short, help = DESCRIPTION_HELP)]
    pub description: Option<String>,
}

pub fn execute_do_release(ctx: &Cli, args: &ReleaseCompletedIssuesArgs) -> Result<(), AppError> {
    do_release(ctx, args)
}

pub fn do_release(ctx: &Cli, args: &ReleaseCompletedIssuesArgs) -> Result<(), AppError> {
    let now: DateTime<Utc> = Utc::now();
    let jql = match args.issue_jql.clone() {
        None => {
            format!(
                "(project = {0} AND status = Done) and (fixVersion is EMPTY)",
                args.project.clone()
            )
        }
        Some(s) => s,
    };

    let issuelist = do_search_issues(
        ctx,
        &SearchIssuesArgs {
            jql,
            unpaginate: true,
            page_size: 50,
            page_start_idx: 0,
        },
    )?;
    if issuelist.is_empty() {
        return Err(AppError::NoIssuesFound);
    }
    let releaseresult = releases::do_create_release(
        ctx,
        &CreateReleaseArgs {
            project: args.project.clone(),
            name: args.release_name.clone(),
            description: args.description.clone(),
            start_date: None,
            release_date: Some(now.format("%Y-%m-%d").to_string()),
        },
    )?;
    match releaseresult {
        Some(release) => {
            issuelist.iter().for_each(|issue| {
                match execute_update_issue(
                    ctx,
                    &UpdateIssueArgs {
                        name: issue.key.clone(),
                        fix_version: Some(release.id.clone()),
                        related_version: None,
                        use_version_id: true,
                    },
                ) {
                    Ok(_) => {}
                    Err(_) => {
                        warn!("could not add fix version to issue {0}", issue.key.clone())
                    }
                }
            });
            do_update_release(
                ctx,
                &UpdateReleaseArgs {
                    name: None,
                    description: None,
                    start_date: None,
                    release_date: None,
                    is_released: Some(true),
                    by_id: Some(true),
                    release: release.id.clone(),
                    project: args.project.clone(),
                },
            )?;
            Ok(())
        }
        _ => Err(CouldNotCreateRelease),
    }
}

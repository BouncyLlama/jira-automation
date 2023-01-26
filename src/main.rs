extern crate core;

mod lib;

use crate::lib::commands::*;
use std::error::Error;
use clap::{Parser, Subcommand};
use env_logger::Env;
use log::error;
use crate::lib::util::Format;

#[derive(Parser)]
#[command(author, version, about, long_about = None)]
pub struct Cli {
    #[arg(long, short, help = "jira personal access token")]
    auth_token: String,

    #[clap(value_enum)]
    #[arg(long, default_value_t = Format::Csv, help = "how returned items should be formatted")]
    output_format: Format,
    #[arg(long, short, help = "email address the auth token belongs to")]
    user_email: String,
    #[arg(long, short, help = "base url of the jira instance ex http://potato.atlassian.net")]
    base_jira_url: String,
    #[command(subcommand)]
    command: Option<Commands>,
}

#[derive(Subcommand)]
enum Commands {
    /// list and optionally filter releases
    ListReleases(releases::ListReleasesArgs),
    /// create a new release
    CreateRelease(releases::CreateReleaseArgs),
    /// delete a release and optionally update tickets to point to a different one
    DeleteRelease(releases::DeleteReleaseArgs),
    /// update a release
    UpdateRelease(releases::UpdateReleaseArgs),
    /// list possible transitions for specified issue
    ListIssueTransitions(issues::ListIssueTransitionsArgs),
    /// transition issue
    TransitionIssue(issues::TransitionIssueArgs),
    /// update an issue
    UpdateIssue(issues::UpdateIssueArgs),
    /// jql search for issues
    SearchIssues(issues::SearchIssuesArgs),
}


fn main() -> Result<(), Box<dyn Error>> {
    let cli = Cli::parse();

    env_logger::Builder::from_env(Env::default().default_filter_or("warn")).init();


    // You can check for the existence of subcommands, and if found use their
    // matches just as you would the top level cmd
    let result = match &cli.command {
        Some(Commands::ListReleases(release_args)) => releases::execute_list_releases(&cli, release_args),
        Some(Commands::CreateRelease(args)) => releases::execute_create_release(&cli, args),
        Some(Commands::DeleteRelease(args)) => releases::execute_delete_release(&cli, args),
        Some(Commands::UpdateRelease(args)) => releases::execute_update_release(&cli, args),
        Some(Commands::ListIssueTransitions(args)) => issues::execute_list_transitions(&cli, args),
        Some(Commands::TransitionIssue(args)) => issues::execute_transition_issue(&cli, args),
        Some(Commands::UpdateIssue(args)) => issues::execute_update_issue(&cli, args),
        Some(Commands::SearchIssues(args)) => issues::execute_search_issues(&cli, args),

        None => { Ok(()) }
    };

    match result {
        Ok(_) => { result }
        Err(e) => {
            error!("{:?}",e);
            Err(e)
        }
    }


    // Continued program logic goes here...
}




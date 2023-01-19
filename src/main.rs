mod lib;

use lib::commands::*;
use std::borrow::Borrow;
use std::error::Error;
use std::io;
use std::path::PathBuf;
use serde::Serialize;
use serde::Deserialize;
use clap::{Parser, Subcommand};
use env_logger::Env;
use log::{debug, error, log_enabled, info, Level};
use crate::lib::commands::releases::ListReleasesArgs;
use crate::lib::util::Format;

#[derive(Parser)]
#[command(author, version, about, long_about = None)]
pub struct Cli {
    #[arg(long, short, help = "jira personal access token")]
    authToken: String,

    #[clap(value_enum)]
    #[arg(long, default_value_t = Format::csv, help = "how returned items should be formatted")]
    output_format: Format,
    #[arg(long, short, help = "email address the auth token belongs to")]
    userEmail: String,
    #[arg(long, short, help = "base url of the jira instance ex http://potato.atlassian.net")]
    baseJiraUrl: String,
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
    UpdateIssue(issues::UpdateIssueArgs)
}


fn main() -> Result<(), Box<dyn Error>> {
    let cli = Cli::parse();

    env_logger::Builder::from_env(Env::default().default_filter_or("warn")).init();


    // You can check for the existence of subcommands, and if found use their
    // matches just as you would the top level cmd
    match &cli.command {
        Some(Commands::ListReleases(releaseArgs)) => releases::execute_list_releases(&cli, releaseArgs),
        Some(Commands::CreateRelease(args)) => releases::execute_create_release(&cli, args),
        Some(Commands::DeleteRelease(args)) => releases::execute_delete_release(&cli, args),
        Some(Commands::UpdateRelease(args)) => releases::execute_update_release(&cli, args),
        Some(Commands::ListIssueTransitions(args)) => issues::execute_list_transitions(&cli, args),
        Some(Commands::TransitionIssue(args)) => issues::execute_transition_issue(&cli, args),
        Some(Commands::UpdateIssue(args)) => issues::execute_update_issue(&cli, args),

        None => { Ok(()) }
    }


    // Continued program logic goes here...
}




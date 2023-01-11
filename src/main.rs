
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
use struct_field_names_as_array::FieldNamesAsArray;
use crate::lib::commands::releases::list_release_args;
use crate::lib::util::Format;

#[derive(Parser)]
#[command(author, version, about, long_about = None)]
pub struct Cli {

    #[arg( long)]
    authToken: String,

    #[clap(value_enum)]
    #[arg( long)]
    output_format:Format,
    #[arg( long)]
    userEmail: String,
    #[arg( long)]
    baseJiraUrl: String,
    #[command(subcommand)]
    command: Option<Commands>,
}

#[derive(Subcommand)]
enum Commands {
    /// Fetch jira releases
    ListReleases(releases::list_release_args),
    CreateRelease(releases::CreateReleaseArgs),
    DeleteRelease(releases::DeleteReleaseArgs),
    UpdateRelease(releases::UpdateReleaseArgs)
}


fn main() -> Result<(), Box<dyn Error>> {
    let cli = Cli::parse();

    env_logger::Builder::from_env(Env::default().default_filter_or("warn")).init();


    // You can check for the existence of subcommands, and if found use their
    // matches just as you would the top level cmd
    match &cli.command {
        Some(Commands::ListReleases(releaseArgs)) => releases::execute_list_releases(&cli, releaseArgs),
        Some(Commands::CreateRelease(args))=>releases::execute_create_release(&cli,args),
        Some(Commands::DeleteRelease(args))=>releases::execute_delete_release(&cli,args),
        Some(Commands::UpdateRelease(args))=>releases::execute_update_release(&cli,args),

        None => { Ok(()) }
    }


    // Continued program logic goes here...
}




use std::collections::HashMap;
use std::error::Error;
use crate::Cli;
use crate::lib::commands::issues::Transition;
use crate::lib::util;
use clap::Parser;
use super::*;

#[derive(Parser, Clone)]
#[command()]
pub struct TransitionIssueArgs {
    #[arg(long, short, help = issueNameHelp)]
    pub(crate) name: String,
    #[arg(long, short, help = transitionHelp)]

    pub(crate) transition: String,
    #[arg(long, short, help = byIdHelp, default_value_t = false)]
    pub(crate) useTransitionId: bool,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
struct TransitionIssueRequest {
    pub(crate) transition: ReqTransition,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
struct ReqTransition {
    pub(crate) id: String,
}


pub fn execute_transition_issue(ctx: &Cli, args: &TransitionIssueArgs) -> Result<(), Box<dyn Error>> {
    let mut id = args.transition.clone();
    if args.useTransitionId {} else {
        id = get_transition_id(ctx, args)?;
    }
    do_transition(ctx, id, args.name.clone())
}

fn do_transition(ctx: &Cli, transitionId: String, issue: String) -> Result<(), Box<dyn Error>> {
    let req = TransitionIssueRequest {
        transition: ReqTransition { id: transitionId }
    };
    let reqUrl = format!("{}/rest/api/3/issue/{}/transitions", ctx.baseJiraUrl, issue);
    let result = util::doPost::<(), TransitionIssueRequest>(&reqUrl, ctx, &req)?;
    Ok(())
}

fn get_transition_id(ctx: &Cli, args: &TransitionIssueArgs) -> Result<(String), Box<dyn Error>> {
    let results = do_list_transitions(ctx, &ListIssueTransitionsArgs { name: args.name.clone(), includeUnavailable: false })?;
    let filtered: Vec<&Transition> = results.iter().filter(|t| t.name == args.transition).collect();
    if filtered.len() != 1 {
        Err(Box::try_from("Transition name does not match any known transition").unwrap())
    } else {
        Ok((filtered[0].id.clone()))
    }
}
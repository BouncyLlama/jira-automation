use std::collections::HashMap;
use crate::Cli;
use crate::lib::commands::issues::Transition;
use crate::lib::util;
use clap::Parser;
use super::*;
#[derive(Parser, Clone)]
#[command()]
pub struct ListIssueTransitionsArgs {
    #[arg(long, short, help = issueNameHelp)]
    pub(crate) name: String,
    #[arg(long, short, help = includeUnavailableHelp, default_value_t = false)]
    pub(crate) includeUnavailable: bool,
}

#[derive(Deserialize,Serialize,Debug,Clone)]
struct ListTransitionsResponse{
    pub(crate) transitions:Vec<Transition>
}
pub fn execute_list_transitions(ctx: &Cli, args: &ListIssueTransitionsArgs) -> Result<(), Box<dyn std::error::Error>> {
    let values = do_list_transitions(ctx, args)?;
    util::formatPrint::<Transition>(values, ctx.output_format)?;


    Ok(())
}

pub(crate) fn do_list_transitions(ctx: &Cli, args: &ListIssueTransitionsArgs) -> Result<Vec<Transition>, Box<dyn std::error::Error>> {
    let (reqUrl, queryParams) = assemble_query(ctx, args);
    let url = reqUrl.clone();
    let mut values: Vec::<Transition>;
    values = vec![];
    let mut res = util::doGet::<ListTransitionsResponse, HashMap<&str, String>>(&reqUrl, ctx, queryParams)?;


    values.append(&mut res.transitions);

    Ok(values)
}

fn execute_list_issue_transitions(ctx: &Cli, args: &ListIssueTransitionsArgs) {
    let query = assemble_query(ctx, args);
}

fn assemble_query<'a>(ctx: &Cli, args: &'a ListIssueTransitionsArgs) -> (String, HashMap<&'a str, String>) {
    let margs = args.clone();
    let reqUrl = format!("{}/rest/api/3/issue/{}/transitions", ctx.baseJiraUrl, args.name);

    let mut queryParams = HashMap::<&str, String>::new();
    queryParams.insert("includeUnavailable", args.includeUnavailable.to_string());
    return (reqUrl.clone(), queryParams.clone());
}

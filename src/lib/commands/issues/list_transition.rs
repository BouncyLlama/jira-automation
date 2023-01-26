use super::*;
use crate::lib::commands::issues::Transition;
use crate::lib::{util, AppError};
use crate::Cli;
use clap::Parser;
use std::collections::HashMap;
#[derive(Parser, Clone)]
#[command()]
pub struct ListIssueTransitionsArgs {
    #[arg(long, short, help = ISSUE_NAME_HELP)]
    pub(crate) name: String,
    #[arg(long, short, help = INCLUDE_UNAVAILABLE_HELP, default_value_t = false)]
    pub(crate) include_unavailable: bool,
}

#[derive(Deserialize, Serialize, Debug, Clone)]
struct ListTransitionsResponse {
    pub(crate) transitions: Vec<Transition>,
}
pub fn execute_list_transitions(
    ctx: &Cli,
    args: &ListIssueTransitionsArgs,
) -> Result<(), AppError> {
    let values = do_list_transitions(ctx, args)?;
    util::format_print::<Transition>(values, ctx.output_format)?;

    Ok(())
}

pub(crate) fn do_list_transitions(
    ctx: &Cli,
    args: &ListIssueTransitionsArgs,
) -> Result<Vec<Transition>, AppError> {
    let (req_url, query_params) = assemble_query(ctx, args);
    let mut values: Vec<Transition>;
    values = vec![];
    let mut res = util::do_get::<ListTransitionsResponse, HashMap<&str, String>>(
        &req_url,
        ctx,
        query_params,
    )?;

    values.append(&mut res.transitions);

    Ok(values)
}

fn assemble_query<'a>(
    ctx: &Cli,
    args: &'a ListIssueTransitionsArgs,
) -> (String, HashMap<&'a str, String>) {
    let req_url = format!(
        "{}/rest/api/3/issue/{}/transitions",
        ctx.base_jira_url, args.name
    );

    let mut query_params = HashMap::<&str, String>::new();
    query_params.insert("includeUnavailable", args.include_unavailable.to_string());
    (req_url, query_params.clone())
}

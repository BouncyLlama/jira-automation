use std::error::Error;
use clap::Parser;
use crate::Cli;
use serde::Serialize;
use crate::lib::commands::issues::{Issue, PaginatedIssues};
use crate::lib::util;
use crate::lib::util::Format;
use crate::lib::util::Format::Json;

#[derive(Parser, Clone)]
#[command()]
pub struct SearchIssuesArgs {
    pub jql: String,
    #[arg(
    long,
    short,
    default_value_t = false,
    help = "automatically query until all pages have been obtained"
    )]
    pub(crate) unpaginate: bool,
    #[arg(long, default_value_t = 50, help = "how many items to return")]
    pub(crate) page_size: u64,
    #[arg(long, default_value_t = 0, help = "item index to begin paging at")]
    pub(crate) page_start_idx: u64,
}

#[derive(Serialize, Debug)]
#[serde(rename_all = "camelCase")]
pub struct SearchIssuesRequest {
    jql: String,
    start_at: u64,
    max_results: u64,
    fields: Vec<&'static str>,
}

impl SearchIssuesRequest {
    fn set_start_idx(&mut self, new: u64) {
        self.start_at = new;
    }
}

#[derive(Serialize, Debug)]
struct CsvCompatibleIssue {
    pub key: String,
    pub id: String,
    pub summary: String,
    pub status: String,
    pub fix_versions: String,
}


pub fn execute_search_issues(cli: &Cli, args: &SearchIssuesArgs) -> Result<(), Box<dyn std::error::Error>> {
    do_search_issues(cli, args)
}

pub fn do_search_issues(cli: &Cli, args: &SearchIssuesArgs) -> Result<(), Box<dyn Error>> {
    let mut req = SearchIssuesRequest {
        jql: args.jql.clone(),
        start_at: args.page_start_idx,
        max_results: args.page_size,
        fields: vec!["summary", "status", "fixVersions"],
    };
    let results = page_loop(cli, &mut req)?;
    match cli.output_format {
        Format::Csv => {
            let csvresults: Vec<CsvCompatibleIssue> = results.iter().map(|r|  {
                CsvCompatibleIssue {
                    key: r.key.clone(),
                    id: r.id.clone(),
                    summary: r.fields.summary.clone(),
                    status: r.fields.status.name.clone(),
                    fix_versions: r.fields.fix_versions.iter().map(|v| v.name.clone()).collect::<Vec<String>>().join(",")
                }
            }).collect();
            util::format_print(csvresults, cli.output_format)
        }
        Json => {
            util::format_print(results, cli.output_format)
        }
    }
}

fn page_loop(ctx: &Cli, request: &mut SearchIssuesRequest) -> Result<Vec<Issue>, Box<dyn std::error::Error>> {
    let url = format!("{}/rest/api/3/search", ctx.base_jira_url);
    let result = util::do_post::<PaginatedIssues, SearchIssuesRequest>(&url, ctx, request)?;
    let mut issues: Vec<Issue> = vec![];
    if let Some(mut x) = result {
        let remainder = x.total % request.max_results;
        let mut pages = x.total / request.max_results;
        if remainder > 0 {
            pages += 1;
        }
        issues.append(&mut x.issues);
        for idx in 1..pages {
            request.set_start_idx(idx);
            let newpage = util::do_post::<PaginatedIssues, SearchIssuesRequest>(&url.to_string(), ctx, request)?;
            if let Some(mut page) = newpage {
                issues.append(&mut page.issues);
            }
        }
    }

    Ok(issues)
}
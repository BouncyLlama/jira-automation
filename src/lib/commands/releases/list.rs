use super::*;
use crate::lib::commands::releases::{PaginatedReleases, Release};
use crate::lib::util;
use crate::Cli;
use clap::Parser;
use std::collections::HashMap;

#[derive(Parser, Clone)]
#[command()]
pub struct ListReleasesArgs {
    #[arg(long, short, help = PROJECT_HELP)]
    pub(crate) project: String,
    #[arg(
        long,
        short,
        help = "optionally filter results; substring match in the name and description fields"
    )]
    pub(crate) filter: Option<String>,
    #[arg(
        long,
        short,
        default_value_t = false,
        help = "automatically query until all pages have been obtained"
    )]
    pub(crate) unpaginate: bool,
    #[arg(long, default_value_t = 50, help = "how many items to return")]
    pub(crate) page_size: i64,
    #[arg(long, default_value_t = 0, help = "item index to begin paging at")]
    pub(crate) page_start_idx: i64,
}

impl ListReleasesArgs {
    fn update_start_idx(&mut self, new: i64) {
        self.page_start_idx = new;
    }
}

pub fn execute_list_releases(ctx: &Cli, args: &ListReleasesArgs) -> Result<(), AppError> {
    let values = do_list_releases(ctx, args)?;
    util::format_print::<Release>(values, ctx.output_format)?;

    Ok(())
}

pub(crate) fn do_list_releases(
    ctx: &Cli,
    args: &ListReleasesArgs,
) -> Result<Vec<Release>, AppError> {
    let (req_url, query_params) = assemble_query(ctx, args);
    let url = req_url.clone();
    let mut values: Vec<Release>;
    values = vec![];
    let mut res = util::do_get::<PaginatedReleases<Release>, HashMap<&str, String>>(
        &req_url,
        ctx,
        query_params,
    )?;

    if args.unpaginate {
        let arsgs = page_loop(res.total as i64, args);
        for arg in arsgs {
            let mut res =
                util::do_get::<PaginatedReleases<Release>, HashMap<&str, String>>(&url, ctx, arg)?;
            values.append(&mut res.values);
        }
    } else {
        values.append(&mut res.values);
    }
    Ok(values)
}

fn assemble_query<'a>(ctx: &Cli, args: &'a ListReleasesArgs) -> (String, HashMap<&'a str, String>) {
    let margs = args.clone();
    let req_url = format!(
        "{}/rest/api/3/project/{}/version",
        ctx.base_jira_url, args.project
    );

    let query_params = args_to_query_params(margs);
    (req_url, query_params.clone())
}

fn args_to_query_params(args: ListReleasesArgs) -> HashMap<&'static str, String> {
    let mut query_params = HashMap::<&str, String>::new();
    query_params.insert("startAt", args.page_start_idx.to_string());
    query_params.insert("maxResults", args.page_size.to_string());
    if args.filter.is_some() {
        query_params.insert(
            "query",
            (*args.filter.as_ref().unwrap().clone()).parse().unwrap(),
        );
    }
    query_params.clone()
}

fn page_loop(total_results: i64, args: &ListReleasesArgs) -> Vec<HashMap<&str, String>> {
    let remainder = total_results % args.page_size;
    let mut pages = total_results / args.page_size;
    let mut startidx = args.page_start_idx;
    if remainder > 0 {
        pages += 1;
    }
    let mut requests: Vec<HashMap<&str, String>>;
    requests = vec![];
    for _ in 0..pages {
        let mut newargs = args.clone();
        newargs.update_start_idx(startidx);
        requests.push(args_to_query_params(newargs));
        startidx = args.page_size + startidx;
    }
    return requests;
}

#[cfg(test)]
mod tests {
    use super::super::*;

    use crate::lib::util::Format;
    use crate::Cli;
    use httptest::{matchers::*, responders::*, Expectation, ServerPool};

    static SERVER_POOL: ServerPool = ServerPool::new(2);

    #[test]
    fn list_releases_paginated() -> Result<(), Box<dyn std::error::Error>> {
        let server = SERVER_POOL.get_server();

        let ctx = Cli {
            auth_token: "".to_string(),

            output_format: Format::Csv,
            user_email: "".to_string(),
            base_jira_url: server.url("").to_string(),
            command: None,
        };
        let args = ListReleasesArgs {
            project: "foo".to_string(),
            filter: None,
            unpaginate: false,
            page_size: 1,
            page_start_idx: 0,
        };
        let resp = PaginatedReleases {
            total: 2,
            start_at: 0,
            is_last: true,
            values: vec![
                Release {
                    id: "1".to_string(),
                    description: None,
                    name: None,
                    archived: false,
                    released: false,
                    release_date: None,
                    overdue: None,
                    user_release_date: None,
                    project_id: 0,
                },
                Release {
                    id: "2".to_string(),
                    description: None,
                    name: None,
                    archived: false,
                    released: false,
                    release_date: None,
                    overdue: None,
                    user_release_date: None,
                    project_id: 0,
                },
            ],
        };
        // Start a server running on a local ephemeral port.
        // Configure the server to expect a single GET /foo request and respond
        // with a 200 status code.
        server.expect(
            Expectation::matching(any()).respond_with(json_encoded(serde_json::json!(resp))),
        );

        let res = do_list_releases(&ctx, &args).unwrap();
        insta::assert_debug_snapshot!(res);
        Ok(())
    }

    #[test]
    fn list_releases_unpaginated() -> Result<(), Box<dyn std::error::Error>> {
        let server = SERVER_POOL.get_server();

        let ctx = Cli {
            auth_token: "".to_string(),

            output_format: Format::Csv,
            user_email: "".to_string(),
            base_jira_url: server.url("").to_string(),
            command: None,
        };
        let args = ListReleasesArgs {
            project: "foo".to_string(),
            filter: None,
            unpaginate: true,
            page_size: 1,
            page_start_idx: 0,
        };
        let resp1 = PaginatedReleases {
            total: 2,
            start_at: 0,
            is_last: false,
            values: vec![Release {
                id: "1".to_string(),
                description: None,
                name: None,
                archived: false,
                released: false,
                release_date: None,
                overdue: None,
                user_release_date: None,
                project_id: 0,
            }],
        };
        let resp2 = PaginatedReleases {
            total: 2,
            start_at: 1,
            is_last: true,
            values: vec![Release {
                id: "2".to_string(),
                description: None,
                name: None,
                archived: false,
                released: false,
                release_date: None,
                overdue: None,
                user_release_date: None,
                project_id: 0,
            }],
        };
        // Start a server running on a local ephemeral port.
        // Configure the server to expect a single GET /foo request and respond
        // with a 200 status code.
        server.expect(
            Expectation::matching(request::query(url_decoded(contains(("startAt", "0")))))
                .times(2)
                .respond_with(json_encoded(serde_json::json!(resp2))),
        );
        server.expect(
            Expectation::matching(request::query(url_decoded(contains(("startAt", "1")))))
                .respond_with(json_encoded(serde_json::json!(resp1))),
        );

        // server.expect(
        //     Expectation::matching(request::query(url_decoded(contains(("startAt", any()))))).respond_with(json_encoded(serde_json::json!(resp2))), );

        let res = do_list_releases(&ctx, &args)?;

        insta::assert_debug_snapshot!(res);
        Ok(())
    }
}

use std::collections::HashMap;
use crate::Cli;
use crate::lib::commands::releases::{PaginatedReleases, Release};
use crate::lib::util;
use clap::Parser;
use super::*;

#[derive(Parser, Clone)]
#[command()]
pub struct ListReleasesArgs {
    #[arg(long, short, help = projectHelp)]
    pub(crate) project: String,
    #[arg(long, short, help = "optionally filter results; substring match in the name and description fields")]
    pub(crate) filter: Option<String>,
    #[arg(long, short, default_value_t = false, help = "automatically query until all pages have been obtained")]
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

pub fn execute_list_releases(ctx: &Cli, args: &ListReleasesArgs) -> Result<(), Box<dyn std::error::Error>> {
    let values = do_list_releases(ctx, args)?;
    util::formatPrint::<Release>(values, ctx.output_format)?;


    Ok(())
}

pub(crate) fn do_list_releases(ctx: &Cli, args: &ListReleasesArgs) -> Result<Vec<Release>, Box<dyn std::error::Error>> {
    let (reqUrl, queryParams) = assemble_query(ctx, args);
    let url = reqUrl.clone();
    let mut values: Vec::<Release>;
    values = vec![];
    let mut res = util::doGet::<PaginatedReleases<Release>, HashMap<&str, String>>(&reqUrl, ctx, queryParams)?;

    if args.unpaginate {
        let arsgs = page_loop(res.total as i64, args);
        for arg in arsgs {
            let mut res = util::doGet::<PaginatedReleases<Release>, HashMap<&str, String>>(&url, ctx, arg)?;
            values.append(&mut res.values);
        }
    } else {
        values.append(&mut res.values);
    }
    Ok(values)
}

fn assemble_query<'a>(ctx: &Cli, args: &'a ListReleasesArgs) -> (String, HashMap<&'a str, String>) {
    let margs = args.clone();
    let reqUrl = format!("{}/rest/api/3/project/{}/version", ctx.baseJiraUrl, args.project);

    let queryParams = args_to_query_params(margs);
    return (reqUrl.clone(), queryParams.clone());
}

fn args_to_query_params(args: ListReleasesArgs) -> HashMap<&'static str, String> {
    let mut queryParams = HashMap::<&str, String>::new();
    queryParams.insert("startAt", args.page_start_idx.to_string());
    queryParams.insert("maxResults", args.page_size.to_string());
    if args.filter.is_some() {
        queryParams.insert("query", (*args.filter.as_ref().unwrap().clone().to_string()).parse().unwrap());
    }
    return queryParams.clone();
}


fn page_loop(totalResults: i64, args: &ListReleasesArgs) -> Vec<HashMap<&str, String>> {
    let remainder = totalResults % args.page_size;
    let mut pages = totalResults / args.page_size;
    let mut startidx = args.page_start_idx;
    if remainder > 0 {
        pages += 1;
    }
    let mut requests: Vec::<HashMap<&str, String>>;
    requests = vec![];
    for x in 0..pages {
        let mut newargs = args.clone();
        newargs.update_start_idx(startidx);
        requests.push(args_to_query_params(newargs));
        startidx = args.page_size + startidx;
    }
    return requests;
}


#[cfg(test)]
mod tests {
    use super::*;
    use super::super::*;

    use std::collections::HashMap;
    use crate::Cli;
    use crate::lib::util::Format;
    use httpmock::prelude::*;
    use serde_json::json;

    #[test]
    fn list() -> Result<(), Box<dyn std::error::Error>> {
        let ctx = Cli {
            authToken: "".to_string(),

            output_format: Format::csv,
            userEmail: "".to_string(),
            baseJiraUrl: "asdf".to_string(),
            command: None,
        };
        let args = ListReleasesArgs {
            project: "foo".to_string(),
            filter: None,
            unpaginate: false,
            page_size: 50,
            page_start_idx: 0,
        };
        let (url, args) = assemble_query(&ctx, &args);
        assert!(url.contains("asdf") && url.contains("foo"));
        assert_eq!(2, args.len());
        assert!(args.contains_key("startAt"));
        assert!(args.contains_key("maxResults"));
        assert_eq!("50", args.get("maxResults").unwrap());
        assert_eq!("0", args.get("startAt").unwrap());

        Ok(())
    }

    #[test]
    fn pageloop() -> Result<(), Box<dyn std::error::Error>> {
        let ctx = Cli {
            authToken: "".to_string(),

            output_format: Format::csv,
            userEmail: "".to_string(),
            baseJiraUrl: "asdf".to_string(),
            command: None,
        };
        let args = ListReleasesArgs {
            project: "foo".to_string(),
            filter: None,
            unpaginate: true,
            page_size: 50,
            page_start_idx: 0,
        };
        let result = page_loop(100, &args);
        assert_eq!(2, result.len());
        assert_eq!("0", result.get(0).unwrap().get("startAt").unwrap());
        assert_eq!("50", result.get(1).unwrap().get("startAt").unwrap());

        Ok(())
    }
}
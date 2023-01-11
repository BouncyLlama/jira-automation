use std::fmt::Write;
use super::super::*;
use clap::Parser;
use log::error;
use clap::Subcommand;
use clap::ArgGroup;
use log::info;
use log::warn;
use log::trace;
use log::{debug, LevelFilter};
use clap::arg;
use crate::lib::util;
use serde::Serialize;
use serde::Deserialize;
use std::{io, thread};
use std::borrow::Borrow;
use std::collections::HashMap;
use std::io::BufRead;
use std::time::Duration;
use crate::Cli;

mod test_releases;

#[derive(Parser, Clone)]
#[command(group(ArgGroup::new("listreleasesgroup").required(true)
.args(["project"])))]
pub struct list_release_args {
    #[arg(long)]
    pub(crate) project: String,
    #[arg(long)]
    pub(crate) filter: Option<String>,
    #[arg(long)]
    unpaginate: bool,
    #[arg(long, default_value_t = 50)]
    page_size: i64,
    #[arg(long, default_value_t = 0)]
    page_start_idx: i64,
}

#[derive(Parser, Clone)]
#[command()]
pub struct CreateReleaseArgs {
    #[arg(long)]
    pub(crate) project: String,
    #[arg(long)]
    pub(crate) name: String,
    #[arg(long)]
    pub(crate) description: Option<String>,
    #[arg(long)]
    pub(crate) startDate: Option<String>,
    #[arg(long)]
    pub(crate) releaseDate: Option<String>,
}
#[derive(Parser, Clone)]
#[command()]
pub struct UpdateReleaseArgs {


    #[arg(long)]
    pub(crate) name: Option<String>,
    #[arg(long)]
    pub(crate) description: Option<String>,
    #[arg(long)]
    pub(crate) startDate: Option<String>,
    #[arg(long)]
    pub(crate) releaseDate: Option<String>,
    #[arg(long)]
    pub(crate) is_released: Option<bool>,
    #[arg(long)]
    pub(crate) byId: Option<bool>,
    #[arg(long)]
    pub(crate) release:String,
    #[arg(long)]
    pub(crate) project: String,
}
#[derive(Parser, Clone)]
#[command()]
pub struct DeleteReleaseArgs {
    #[arg(long)]
    pub(crate) release: String,
    #[arg(long)]
    pub(crate) project: String,
    #[arg(long)]
    pub(crate) byId: Option<bool>,
    #[arg(long)]
    pub(crate) replaceFixVersion: Option<String>,
    #[arg(long)]
    pub(crate) replaceAffectedVersion: Option<String>,

}
impl DeleteReleaseArgs{
    fn set_release(&mut self, new:String){
        self.release = new;
    }
}
impl UpdateReleaseArgs{
    fn set_release(&mut self, new:String){
        self.release = new;
    }
}

impl list_release_args {
    fn update_start_idx(&mut self, new: i64) {
        self.page_start_idx = new;
    }
}

#[derive(Serialize, Deserialize, Debug)]
pub struct Release {
    id: String,
    description: Option<String>,
    name: Option<String>,
    archived: bool,
    released: bool,
    releaseDate: Option<String>,
    overdue: Option<bool>,
    userReleaseDate: Option<String>,
    projectId: u64,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct PaginatedStuff<T> {
    pub(crate) total: u64,
    pub(crate) startAt: u64,
    pub(crate) isLast: bool,
    pub(crate) values: Vec<T>,
}

pub fn execute_create_release(ctx: &Cli, args: &CreateReleaseArgs) -> Result<(), Box<dyn std::error::Error>> {
    let reqUrl = format!("{}/rest/api/3/version", ctx.baseJiraUrl, );
    let result = util::doPost::<Release, HashMap<&str, String>>(&reqUrl, ctx, &(assemble_create_args(args.clone())))?;
    Ok(())
}

pub fn execute_delete_release(ctx: &Cli, args:  &DeleteReleaseArgs) -> Result<(), Box<dyn std::error::Error>> {
    let mut mutargs = args.clone();
    if args.byId.is_some() && args.byId.unwrap() == true {
        let reqUrl = format!("{}/rest/api/3/version/{}/removeAndSwap", ctx.baseJiraUrl, args.release);

        util::doPost::<(), HashMap<&str, String>>(&reqUrl, ctx, &(assemble_delete_args(args.clone())))?;

    }else{
        let id = get_id_from_name(ctx, args.project.clone(), args.release.clone())?;
        let reqUrl = format!("{}/rest/api/3/version/{}/removeAndSwap", ctx.baseJiraUrl, id);

        debug!("found release {} for name {}",id,args.release);
        mutargs.set_release(id);
        util::doPost::<(), HashMap<&str, String>>(&reqUrl, ctx, &(assemble_delete_args(mutargs.clone())))?;

    }

    Ok(())
}


pub fn execute_update_release(ctx: &Cli, args:  &UpdateReleaseArgs) -> Result<(), Box<dyn std::error::Error>> {
    let mut mutargs = args.clone();
    if args.byId.is_some() && args.byId.unwrap() == true {
        let reqUrl = format!("{}/rest/api/3/version/{}", ctx.baseJiraUrl, args.release);

        util::doPut::<(), HashMap<&str, String>>(&reqUrl, ctx, &(assemble_update_args(args.clone())))?;

    }else{
        let id = get_id_from_name(ctx, args.project.clone(), args.release.clone())?;
        let reqUrl = format!("{}/rest/api/3/version/{}", ctx.baseJiraUrl, id);

        debug!("found release {} for name {}",id,args.release);
        mutargs.set_release(id);
        util::doPut::<(), HashMap<&str, String>>(&reqUrl, ctx, &(assemble_update_args(mutargs.clone())))?;

    }

    Ok(())
}

fn get_id_from_name(ctx: &Cli, project: String, name: String) -> Result<String, Box<dyn std::error::Error>> {
    let args = list_release_args {
        project,
        filter: Option::from(name),
        unpaginate: true,
        page_size: 100,
        page_start_idx: 0,
    };
    let mut result = do_list_releases(ctx, &args)?;
    if result.len() != 1 {
        Err(Box::try_from("release name matches multiple releases").unwrap())
    } else {
        let id = result.get(0).unwrap().id.clone();
        Ok(id)
    }
}

fn assemble_create_args(args: CreateReleaseArgs) -> HashMap<&'static str, String> {
    let mut params: HashMap<&str, String> = HashMap::new();
    params.insert("name", args.name);
    params.insert("project", args.project);
    if args.description.is_some() {
        params.insert("description", args.description.unwrap());
    }
    if args.startDate.is_some() {
        params.insert("startDate", args.startDate.unwrap());
    }
    if args.releaseDate.is_some() {
        params.insert("releaseDate", args.releaseDate.unwrap());
    }
    return params;
}

fn assemble_delete_args(args: DeleteReleaseArgs) -> HashMap<&'static str, String> {
    let mut params: HashMap<&str, String> = HashMap::new();
    if args.replaceAffectedVersion.is_some() {
        params.insert("moveAffectedIssuesTo", args.replaceAffectedVersion.unwrap());
    }
    if args.replaceFixVersion.is_some() {
        params.insert("moveFixIssuesTo", args.replaceFixVersion.unwrap());
    }

    return params;
}
fn assemble_update_args(args: UpdateReleaseArgs) -> HashMap<&'static str, String> {
    let mut params: HashMap<&str, String> = HashMap::new();
    if args.startDate.is_some() {
        params.insert("startDate", args.startDate.unwrap());
    }
    if args.releaseDate.is_some() {
        params.insert("releaseDate", args.releaseDate.unwrap());
    }
    if args.description.is_some(){
        params.insert("description", args.description.unwrap());

    }
    if args.name.is_some(){
        params.insert("name", args.name.unwrap());

    }
    if args.is_released.is_some(){
        params.insert("released", args.is_released.unwrap().to_string());

    }

    return params;
}
pub fn execute_list_releases(ctx: &Cli, args: &list_release_args) -> Result<(), Box<dyn std::error::Error>> {
    let values = do_list_releases(ctx, args)?;
    util::formatPrint::<Release>(values, ctx.output_format)?;


    Ok(())
}

fn do_list_releases(ctx: &Cli, args: &list_release_args) -> Result<Vec<Release>, Box<dyn std::error::Error>> {
    let (reqUrl, queryParams) = assemble_query(ctx, args);
    let url = reqUrl.clone();
    let mut values: Vec::<Release>;
    values = vec![];
    let mut res = util::doGet::<PaginatedStuff<Release>, HashMap<&str, String>>(&reqUrl, ctx, queryParams)?;

    if args.unpaginate {
        let arsgs = page_loop(res.total as i64, args);
        for arg in arsgs {
            let mut res = util::doGet::<PaginatedStuff<Release>, HashMap<&str, String>>(&url, ctx, arg)?;
            values.append(&mut res.values);
        }
    } else {
        values.append(&mut res.values);
    }
    Ok(values)
}

fn page_loop(totalResults: i64, args: &list_release_args) -> Vec<HashMap<&str, String>> {
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

fn args_to_query_params(args: list_release_args) -> HashMap<&'static str, String> {
    let mut queryParams = HashMap::<&str, String>::new();
    queryParams.insert("startAt", args.page_start_idx.to_string());
    queryParams.insert("maxResults", args.page_size.to_string());
    if args.filter.is_some() {
        queryParams.insert("query", (*args.filter.as_ref().unwrap().clone().to_string()).parse().unwrap());
    }
    return queryParams.clone();
}

fn assemble_query<'a>(ctx: &Cli, args: &'a list_release_args) -> (String, HashMap<&'a str, String>) {
    let margs = args.clone();
    let reqUrl = format!("{}/rest/api/3/project/{}/version", ctx.baseJiraUrl, args.project);

    let queryParams = args_to_query_params(margs);
    return (reqUrl.clone(), queryParams.clone());
}



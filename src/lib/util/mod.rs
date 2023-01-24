use std::any::TypeId;
use std::io;
use serde::{ Serialize};
use serde::de::DeserializeOwned;
use crate::Cli;
use std::option::Option;
use log::{trace};


#[derive(clap::ValueEnum)]
#[derive(Debug, Copy, Clone)]
pub enum Format {
    Csv,
    Json,
}

pub fn format_print<T: Serialize>(items: Vec<T>, format: Format) -> Result<(), Box<dyn std::error::Error>> {
    match format {
        Format::Csv => {
            let mut writer = csv::WriterBuilder::new().has_headers(true).from_writer(io::stdout());
            items.iter().for_each(|i| {
                writer.serialize(i).expect("TODO: panic message");
            });
            writer.flush()?;
        }
        Format::Json => {
            let json = serde_json::to_string(&items).expect("TODO: panic message");
            println!("{}", json);
        }
    }

    Ok(())
}


pub fn do_get<T: DeserializeOwned, S: Serialize>(req_url: &String, ctx: &Cli, query_params: S) -> Result<T, Box<dyn std::error::Error>> {
    let client = reqwest::blocking::Client::new();
    let token = base64::encode(format!("{}:{}", ctx.user_email, ctx.auth_token));

    let res = client
        .get(req_url).query(&query_params)
        .header("Content-Type", "application/json").header("Authorization", format!("Basic {}", token)).send()?;


    Ok(res.json::<T>()?)
}

pub fn do_post<T: DeserializeOwned + 'static, S: Serialize>(req_url: &String, ctx: &Cli, post_body: &S) -> Result<Option<T>, Box<dyn std::error::Error>> {
    let client = reqwest::blocking::Client::new();

    let token = base64::encode(format!("{}:{}", ctx.user_email, ctx.auth_token));
    let res = client
        .post(req_url).body(serde_json::to_string(post_body).unwrap())
        .header("Content-Type", "application/json").header("Authorization", format!("Basic {}", token)).send()?;
    trace!("{:?}", serde_json::json!(post_body));
    trace!("{:?}", res.status());
    let body = res.text()?;
    trace!("{:?}", body);
    if TypeId::of::<T>() == TypeId::of::<()>() {
        Ok(None)
    } else {
        Ok(Some(serde_json::from_str(&*body).unwrap()))
    }
}pub fn do_put<T: DeserializeOwned + 'static, S: Serialize>(req_url: &String, ctx: &Cli, put_body: &S) -> Result<Option<T>, Box<dyn std::error::Error>> {
    let client = reqwest::blocking::Client::new();
    let token = base64::encode(format!("{}:{}", ctx.user_email, ctx.auth_token));
    let res = client
        .put(req_url).body(serde_json::to_string(put_body).unwrap())
        .header("Content-Type", "application/json").header("Authorization", format!("Basic {}", token)).send()?;
    trace!("{:?}", serde_json::json!(put_body));
    trace!("{:?}", res.status());
    let body = res.text()?;
    trace!("{:?}", body);
    if TypeId::of::<T>() == TypeId::of::<()>() {
        Ok(None)
    } else {
        Ok(Some(serde_json::from_str(&*body).unwrap()))
    }
}
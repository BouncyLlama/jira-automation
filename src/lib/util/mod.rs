use std::any::TypeId;
use std::collections::HashMap;
use std::io;
use serde::{Deserialize, Serialize};
use serde::de::DeserializeOwned;
use crate::Cli;
use std::option::Option;
use log::{debug, trace};

pub fn dothething() {}


#[derive(clap::ValueEnum)]
#[derive(Debug, Copy, Clone)]
pub enum Format {
    csv,
    json,
}

pub fn formatPrint<T: Serialize>(items: Vec<T>, format: Format) -> Result<(), Box<dyn std::error::Error>> {
    match format {
        Format::csv => {
            let mut writer = csv::WriterBuilder::new().has_headers(true).from_writer(io::stdout());
            items.iter().for_each(|i| {
                writer.serialize(i).expect("TODO: panic message");
            });
            writer.flush()?;
        }
        Format::json => {
            let json = serde_json::to_string(&items).expect("TODO: panic message");
            println!("{}", json);
        }
    }

    Ok(())
}


pub fn doGet<T: DeserializeOwned, S: Serialize>(reqUrl: &String, ctx: &Cli, queryParams: S) -> Result<(T), Box<dyn std::error::Error>> {
    let client = reqwest::blocking::Client::new();
    let token = base64::encode(format!("{}:{}", ctx.userEmail, ctx.authToken));

    let res = client
        .get(reqUrl).query(&queryParams)
        .header("Content-Type", "application/json").header("Authorization", format!("Basic {}", token)).send()?;


    Ok(res.json::<T>()?)
}

pub fn doPost<T: DeserializeOwned + 'static, S: Serialize>(reqUrl: &String, ctx: &Cli, postBody: &S) -> Result<Option<T>, Box<dyn std::error::Error>> {
    let client = reqwest::blocking::Client::new();

    let token = base64::encode(format!("{}:{}", ctx.userEmail, ctx.authToken));
    let res = client
        .post(reqUrl).body(serde_json::to_string(postBody).unwrap())
        .header("Content-Type", "application/json").header("Authorization", format!("Basic {}", token)).send()?;
    trace!("{:?}", serde_json::json!(postBody));
    trace!("{:?}", res.status());
    let body = res.text()?;
    trace!("{:?}", body);
    if TypeId::of::<T>() == TypeId::of::<()>() {
        Ok(None)
    } else {
        Ok(Some(serde_json::from_str(&*body).unwrap()))
    }
}pub fn doPut<T: DeserializeOwned + 'static, S: Serialize>(reqUrl: &String, ctx: &Cli, postBody: &S) -> Result<Option<T>, Box<dyn std::error::Error>> {
    let client = reqwest::blocking::Client::new();
    let token = base64::encode(format!("{}:{}", ctx.userEmail, ctx.authToken));
    let res = client
        .put(reqUrl).body(serde_json::to_string(postBody).unwrap())
        .header("Content-Type", "application/json").header("Authorization", format!("Basic {}", token)).send()?;
    trace!("{:?}", serde_json::json!(postBody));
    trace!("{:?}", res.status());
    let body = res.text()?;
    trace!("{:?}", body);
    if TypeId::of::<T>() == TypeId::of::<()>() {
        Ok(None)
    } else {
        Ok(Some(serde_json::from_str(&*body).unwrap()))
    }
}
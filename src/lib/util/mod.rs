use super::AppError;
use crate::Cli;
use log::trace;
use serde::de::DeserializeOwned;
use serde::Serialize;
use serde_json::from_str;
use std::any::TypeId;
use std::io;
use std::option::Option;

#[derive(clap::ValueEnum, Debug, Copy, Clone)]
pub enum Format {
    Csv,
    Json,
}

pub fn format_print<T: Serialize>(items: Vec<T>, format: Format) -> Result<(), AppError> {
    match format {
        Format::Csv => {
            let mut writer = csv::WriterBuilder::new()
                .has_headers(true)
                .from_writer(io::stdout());
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

pub fn do_get<T: DeserializeOwned, S: Serialize>(
    req_url: &String,
    ctx: &Cli,
    query_params: S,
) -> Result<T, AppError> {
    let client = reqwest::blocking::Client::new();
    let token = base64::encode(format!("{}:{}", ctx.user_email, ctx.auth_token));

    let res = client
        .get(req_url)
        .query(&query_params)
        .header("Content-Type", "application/json")
        .header("Authorization", format!("Basic {}", token))
        .send()?;
    if !res.status().is_success() {
        return Err(AppError::ApiCallBadStatus(format!(
            "status code {}",
            res.status().as_str()
        )));
    }
    match res.json::<T>() {
        Ok(r) => Ok(r),
        Err(_) => Err(AppError::DeserializationError),
    }
}

pub fn do_post<T: DeserializeOwned + 'static, S: Serialize>(
    req_url: &String,
    ctx: &Cli,
    post_body: &S,
) -> Result<Option<T>, AppError> {
    let client = reqwest::blocking::Client::new();

    let token = base64::encode(format!("{}:{}", ctx.user_email, ctx.auth_token));
    let res = client
        .post(req_url)
        .body(serde_json::to_string(post_body).unwrap())
        .header("Content-Type", "application/json")
        .header("Authorization", format!("Basic {}", token))
        .send()?;

    trace!("{:?}", serde_json::json!(post_body));
    trace!("{:?}", res.status());
    if !res.status().is_success() {
        return Err(AppError::ApiCallBadStatus(format!(
            "status code {}",
            res.status().as_str()
        )));
    }
    let body = res.text()?;

    trace!("{:?}", body);

    if TypeId::of::<T>() == TypeId::of::<()>() {
        Ok(None)
    } else {
        match from_str::<T>(&*body) {
            Ok(r) => Ok(Some(r)),
            Err(_) => Err(AppError::DeserializationError),
        }
    }
}

pub fn do_put<T: DeserializeOwned + 'static, S: Serialize>(
    req_url: &String,
    ctx: &Cli,
    put_body: &S,
) -> Result<Option<T>, AppError> {
    let client = reqwest::blocking::Client::new();
    let token = base64::encode(format!("{}:{}", ctx.user_email, ctx.auth_token));
    let res = client
        .put(req_url)
        .body(serde_json::to_string(put_body).unwrap())
        .header("Content-Type", "application/json")
        .header("Authorization", format!("Basic {}", token))
        .send()?;
    trace!("{:?}", serde_json::json!(put_body));
    let status = res.status().clone();
    let body = res.text()?;
    trace!("{:?}", body);
    if !status.is_success() {
        return Err(AppError::ApiCallBadStatus(format!(
            "status code {}",
            status.as_str()
        )));
    }
    if TypeId::of::<T>() == TypeId::of::<()>() {
        Ok(None)
    } else {
        match from_str::<T>(&*body) {
            Ok(r) => Ok(Some(r)),
            Err(_) => Err(AppError::DeserializationError),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::super::*;
    use std::collections::HashMap;

    use crate::lib::commands::releases::Release;
    use crate::lib::util::{do_get, Format};
    use crate::Cli;
    use httptest::{matchers::*, responders::*, Expectation, ServerPool};

    static SERVER_POOL: ServerPool = ServerPool::new(2);

    #[test]
    fn test_deser_error() -> Result<(), Box<dyn std::error::Error>> {
        let server = SERVER_POOL.get_server();
        let ctx = Cli {
            auth_token: "".to_string(),
            output_format: Format::Csv,
            user_email: "".to_string(),
            base_jira_url: "".to_string(),
            command: None,
        };
        let params: HashMap<&str, &str> = HashMap::new();
        let url = server.url("/foo");
        server.expect(Expectation::matching(any()).respond_with(json_encoded("")));
        let res = do_get::<Release, HashMap<&str, &str>>(&url.to_string(), &ctx, params);

        assert!(res.is_err());
        assert!(matches!(res.err().unwrap(), AppError::DeserializationError));
        Ok(())
    }

    #[test]
    fn test_server_error() -> Result<(), Box<dyn std::error::Error>> {
        let server = SERVER_POOL.get_server();
        let ctx = Cli {
            auth_token: "".to_string(),
            output_format: Format::Csv,
            user_email: "".to_string(),
            base_jira_url: "".to_string(),
            command: None,
        };
        let params: HashMap<&str, &str> = HashMap::new();
        let url = server.url("/foo");
        server.expect(Expectation::matching(any()).respond_with(status_code(500)));
        let res = do_get::<Release, HashMap<&str, &str>>(&url.to_string(), &ctx, params);

        assert!(res.is_err());
        assert!(matches!(res.err().unwrap(), AppError::ApiCallBadStatus(..)));
        Ok(())
    }
}

use crate::{authenticate, util};
use crate::credentials;
use crate::credentials::{Credentials};

use std::io::{Error, ErrorKind};

pub fn tweet(status: &Option<String>, delete: &Option<String>, show: &Option<String>) -> std::io::Result<String> {

    match (status, delete, show) {
        (Some(status), None, None) => do_function(status, &do_tweet),
        (None, Some(id), None) => do_function(id, &do_delete),
        (None, None, Some(id)) => do_function(id, &do_show),
        (_,_,_) => Err(Error::new(ErrorKind::InvalidInput, "Multiple parameters cannot be input to 'tweet' at once")),
    }

}

fn do_function(arg: &str, func: &Fn(&str, Vec<(&str,&str)>, &Credentials) -> std::io::Result<String>) -> std::io::Result<String> {

    let credentials: Credentials = credentials::get_active_credentials()?;

    let params: Vec<(&str,&str)> = [("oauth_consumer_key",credentials.app.application_key.as_str()),("oauth_token",&credentials.user.oauth_token)].to_vec();

    func(arg, params, &credentials)

}

fn do_tweet(_status: &str, _params: Vec<(&str,&str)>, credentials: &Credentials) -> std::io::Result<String> {

    let client = reqwest::Client::new();
    let mut params: Vec<(&str,&str)> = _params.clone();
    let mut req_params: Vec<(&str,&str)> = Vec::new();

    let encoded_status = authenticate::percent_encode_string(_status)?;
    let status: (&str, &str) = ("status", &encoded_status);
    params.push(status);
    req_params.push(status);

    let base_url = "https://api.twitter.com/1.1/statuses/update.json";
    let url = util::get_full_request_url(base_url, req_params)?;

    let headers = authenticate::get_authorization_header("post", base_url, params, &credentials)?;

    let res = client.post(&url)
        .headers(headers)
        .send().unwrap()
        .text();

    Ok(res.unwrap())

}

fn do_delete(id: &str, mut _params: Vec<(&str,&str)>, credentials: &Credentials) -> std::io::Result<String> {

    let client = reqwest::Client::new();
    let params: Vec<(&str,&str)> = _params.clone();
    let req_params: Vec<(&str,&str)> = Vec::new();

    let base_url = &*format!("{}{}.json", "https://api.twitter.com/1.1/statuses/destroy/", id);
    let url = util::get_full_request_url(base_url, req_params)?;

    let headers = authenticate::get_authorization_header("post", base_url, params, &credentials)?;

    let res = client.post(&url)
        .headers(headers)
        .send().unwrap()
        .text();

    Ok(res.unwrap())

}

fn do_show(id: &str, mut _params: Vec<(&str,&str)>, credentials: &Credentials) -> std::io::Result<String> {

    let client = reqwest::Client::new();
    let params: Vec<(&str,&str)> = _params.clone();
    let req_params: Vec<(&str,&str)> = Vec::new();

    let base_url = &*format!("{}{}.json", "https://api.twitter.com/1.1/statuses/show/", id);
    let url = util::get_full_request_url(base_url, req_params)?;

    let headers = authenticate::get_authorization_header("get", base_url, params, &credentials)?;

    let res = client.get(&url)
        .headers(headers)
        .send().unwrap()
        .text();

    Ok(res.unwrap())

}
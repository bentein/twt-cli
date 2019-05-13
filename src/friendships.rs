use crate::{authenticate, util};
use crate::credentials;
use crate::credentials::{Credentials};

pub fn create_friendship(screen_name: &str, notifications: &bool) -> std::io::Result<String> {

    let credentials: Credentials = credentials::get_active_credentials()?;

    let client = reqwest::Client::new();
    let mut params: Vec<(&str,&str)> = [("oauth_consumer_key",credentials.app.application_key.as_str()),("oauth_token",&credentials.user.oauth_token)].to_vec();
    let mut req_params: Vec<(&str,&str)> = Vec::new();

    let screen_name: (&str, &str) = ("screen_name", screen_name);
    params.push(screen_name);
    req_params.push(screen_name);

    let follow: (&str, &str) = ("follow", &notifications.to_string());
    params.push(follow);
    req_params.push(follow);

    let base_url = "https://api.twitter.com/1.1/friendships/create.json";
    let url = util::get_full_request_url(base_url, req_params)?;

    let headers = authenticate::get_authorization_header("post", base_url, params, &credentials)?;

    let res = client.post(&url)
        .headers(headers)
        .send().unwrap()
        .text();

    Ok(res.unwrap())

}

pub fn destroy_friendship(screen_name: &str) -> std::io::Result<String> {

    let credentials: Credentials = credentials::get_active_credentials()?;

    let client = reqwest::Client::new();
    let mut params: Vec<(&str,&str)> = [("oauth_consumer_key",credentials.app.application_key.as_str()),("oauth_token",&credentials.user.oauth_token)].to_vec();
    let mut req_params: Vec<(&str,&str)> = Vec::new();

    let screen_name: (&str, &str) = ("screen_name", screen_name);
    params.push(screen_name);
    req_params.push(screen_name);

    let base_url = "https://api.twitter.com/1.1/friendships/destroy.json";
    let url = util::get_full_request_url(base_url, req_params)?;

    let headers = authenticate::get_authorization_header("post", base_url, params, &credentials)?;

    let res = client.post(&url)
        .headers(headers)
        .send().unwrap()
        .text();

    Ok(res.unwrap())

}
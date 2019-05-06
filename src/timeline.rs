use crate::authenticate;
use crate::credentials;
use crate::credentials::{Credentials};

pub fn get_timeline(user: &Option<String>, count: &Option<String>, max_id: &Option<String>) -> std::io::Result<String> {

    match user {
        Some(_username) => get_some_timeline(user, count, max_id, &get_user_timeline),
        None => get_some_timeline(user, count, max_id, &get_own_timeline),
    }

}

fn get_some_timeline(user: &Option<String>, count: &Option<String>, max_id: &Option<String>,
                     func: &Fn(Vec<(&str,&str)>, Vec<(&str,&str)>, Credentials) -> std::io::Result<String>) -> std::io::Result<String> {

    let credentials: Credentials = credentials::get_active_credentials()?;

    let mut params: Vec<(&str,&str)> = [("oauth_consumer_key",credentials.app.application_key.as_str()),("oauth_token",&credentials.user.oauth_token)].to_vec();
    let mut req_params: Vec<(&str,&str)> = Vec::new();

    let tweet_mode: (&str, &str) = ("tweet_mode", "extended");
    params.push(tweet_mode);
    req_params.push(tweet_mode);

    let user: (&str, &str) = unwrap_parameter("screen_name", user)?;
    if user.0 != "" {
        params.push(user);
        req_params.push(user);
    }

    let count: (&str, &str) = unwrap_parameter("count", count)?;
    if count.0 != "" {
        params.push(count);
        req_params.push(count);
    }

    let max_id: (&str, &str) = unwrap_parameter("max_id", max_id)?;
    if max_id.0 != "" {
        params.push(max_id);
        req_params.push(max_id);
    }

    func(params, req_params, credentials.clone())

}

fn get_user_timeline(params: Vec<(&str,&str)>, req_params: Vec<(&str,&str)>, credentials: Credentials) -> std::io::Result<String> {

    let client = reqwest::Client::new();

    let base_url = "https://api.twitter.com/1.1/statuses/user_timeline.json";
    let url = get_full_request_url(base_url, req_params)?;

    let headers = authenticate::get_authorization_header("get", base_url, params, credentials.clone())?;

    let res = client.get(&url)
        .headers(headers)
        .send().unwrap()
        .text();

    Ok(res.unwrap())

}

fn get_own_timeline(params: Vec<(&str,&str)>, req_params: Vec<(&str,&str)>, credentials: Credentials) -> std::io::Result<String> {

    let client = reqwest::Client::new();

    let base_url = "https://api.twitter.com/1.1/statuses/home_timeline.json";
    let url = get_full_request_url(base_url, req_params)?;

    let headers = authenticate::get_authorization_header("get", base_url, params, credentials.clone())?;

    let res = client.get(&url)
        .headers(headers)
        .send().unwrap()
        .text();

    Ok(res.unwrap())

}

fn get_full_request_url(base_url: &str, req_params: Vec<(&str,&str)>) -> std::io::Result<String> {

    let mut url: String = String::new();
    url.push_str(base_url);

    if !req_params.is_empty() {
        let first_param: &(&str, &str) = req_params.get(0).unwrap();
        url = format!("{}?{}={}", url, first_param.0, first_param.1);

        for param in req_params.iter().skip(1) {
            url = format!("{}&{}={}", url, param.0, param.1);
        }
    }

    Ok(url)

}

fn unwrap_parameter<'l>(name: &'l str, _param: &'l Option<String>) -> std::io::Result<(&'l str, &'l str)> {

    Ok(
        match &_param {
            Some(p) => (name, p),
            None => ("",""),
        }
    )

}
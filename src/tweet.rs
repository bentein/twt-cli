use crate::authenticate;
use std::io::{Error, ErrorKind};

pub fn tweet(status: &Option<String>, delete: &Option<String>, show: &Option<String>) -> std::io::Result<()> {

    match (status, delete, show) {
        (Some(status), None, None) => do_tweet(status),
        (None, Some(id), None) => do_delete(id),
        (None, None, Some(id)) => do_show(id),
        (_,_,_) => Err(Error::new(ErrorKind::InvalidInput, "Multiple parameters cannot be input to 'tweet' at once")),
    }

}

pub fn do_tweet(_status: &str) -> std::io::Result<()> {

    let client = reqwest::Client::new();
    let mut params: Vec<(&str,&str)> = [("oauth_consumer_key","NmnoD1Pew3ho5ZoHITn1JjaLw"),("oauth_token","31216914-ta4Vira4eL8dUl59WH0Q3zPhBiYcc05DgSuaQEGVN")].to_vec();
    let mut req_params: Vec<(&str,&str)> = Vec::new();

    let encoded_status = &*authenticate::percent_encode_string(_status)?;
    let status = ("status", encoded_status);
    params.push(status);
    req_params.push(status);

    let base_url = "https://api.twitter.com/1.1/statuses/update.json";
    let url = get_full_request_url(base_url, req_params)?;

    let headers = authenticate::get_authorization_header("post", base_url, params)?;

    let res = client.post(&url)
        .headers(headers)
        .send().unwrap()
        .text();

    println!("{}", res.unwrap());

    Ok(())

}

pub fn do_delete(id: &str) -> std::io::Result<()> {

    let client = reqwest::Client::new();
    let params: Vec<(&str,&str)> = [("oauth_consumer_key","NmnoD1Pew3ho5ZoHITn1JjaLw"),("oauth_token","31216914-ta4Vira4eL8dUl59WH0Q3zPhBiYcc05DgSuaQEGVN")].to_vec();
    let req_params: Vec<(&str,&str)> = Vec::new();

    let base_url = &*format!("{}{}.json", "https://api.twitter.com/1.1/statuses/destroy/", id);
    let url = get_full_request_url(base_url, req_params)?;

    let headers = authenticate::get_authorization_header("post", base_url, params)?;

    let res = client.post(&url)
        .headers(headers)
        .send().unwrap()
        .text();

    println!("{}", res.unwrap());

    Ok(())

}

pub fn do_show(id: &str) -> std::io::Result<()> {

    let client = reqwest::Client::new();
    let params: Vec<(&str,&str)> = [("oauth_consumer_key","NmnoD1Pew3ho5ZoHITn1JjaLw"),("oauth_token","31216914-ta4Vira4eL8dUl59WH0Q3zPhBiYcc05DgSuaQEGVN")].to_vec();
    let req_params: Vec<(&str,&str)> = Vec::new();

    let base_url = &*format!("{}{}.json", "https://api.twitter.com/1.1/statuses/show/", id);
    let url = get_full_request_url(base_url, req_params)?;

    let headers = authenticate::get_authorization_header("get", base_url, params)?;

    let res = client.get(&url)
        .headers(headers)
        .send().unwrap()
        .text();

    println!("{}", res.unwrap());

    Ok(())

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
use crate::Cli;
use crate::authenticate;

use std::io::prelude::*;
use std::net::{TcpListener, TcpStream};
use std::path::PathBuf;
use std::fs::File;

use webbrowser;

/*
pub fn authorize(_args: Cli) {

    let request_tokens: OauthTokens = get_request_token().unwrap();

    webbrowser::open(&("https://api.twitter.com/oauth/authorize?oauth_token=".to_owned() + &request_tokens.oauth_token))
        .expect("Couldn't open authorization page");

    let listener = TcpListener::bind("127.0.0.1:80").unwrap();

    let oauth_tokens: CallbackParams = match listener.accept() {
        Ok((socket, _addr)) => handle_callback(socket),
        Err(e) => panic!("Couldn't get client: {:?}", e),
    }.unwrap();

    let bearer_tokens = get_bearer_token(oauth_tokens).unwrap();

    match dirs::home_dir() {
        Some(path) => { write_credentials(path, bearer_tokens).unwrap(); },
        None => { panic!("Impossible to get your home dir!"); },
    }

    println!("You have successfully authorized twt-cli to access your account");
}

fn get_request_token() -> std::io::Result<OauthTokens> {

    let client = reqwest::Client::new();

    let url: &str = "https://api.twitter.com/oauth/request_token";

    let headers = authenticate::get_authorization_header("post", url,
        [("oauth_consumer_key","NmnoD1Pew3ho5ZoHITn1JjaLw")].to_vec())?;

    let res = client.post(url)
        .headers(headers)
        .send().unwrap()
        .text();
    
    Ok(OauthTokens::new(res.unwrap()))
}


fn handle_callback(mut stream: TcpStream) -> std::io::Result<CallbackParams> {

    let mut req_str_opt: Option<String> = None;

    let mut buf = [0u8 ;4096];
    match stream.read(&mut buf) {
        Ok(_) => {
            let _req_str = String::from_utf8_lossy(&buf).to_string();
            req_str_opt = Some(_req_str.lines().next().unwrap().to_string());
        },
        Err(e) => println!("Unable to read stream: {}", e),
    }

    let req_params = req_str_opt.unwrap().replace("GET /?", "").replace(" HTTP/1.1", "");
    let params = CallbackParams::new(req_params);
    
    let response = b"HTTP/1.1 200 OK\r\nContent-Type: text/html; charset=UTF-8\r\n\r\n<html target=\"_blank\"><body>You can close this window now.</body></html>\r\n";
    stream.write(response)?;

    Ok(params)
}

fn get_bearer_token(tokens: CallbackParams) -> std::io::Result<OauthTokens> {

    let oauth_consumer_key = "NmnoD1Pew3ho5ZoHITn1JjaLw";
    
    let client = reqwest::Client::new();

    let res = client.post("https://api.twitter.com/oauth/access_token")
        .query(&[("oauth_consumer_key", oauth_consumer_key), ("oauth_token", &tokens.oauth_token), ("oauth_verifier", &tokens.oauth_verifier)])
        .send().unwrap()
        .text();
    
    Ok(OauthTokens::new(res.unwrap()))
}

fn write_credentials(mut path: PathBuf, tokens: OauthTokens) -> std::io::Result<()> {
    path.push(".twtcredentials");
    let mut f = File::create(path).unwrap();

    let oauth_token = "oauth_token=".to_string() + &tokens.oauth_token + "\n";
    let oauth_token_secret = "oauth_token_secret=".to_string() + &tokens.oauth_token_secret;

    f.write(oauth_token.as_bytes()).unwrap();
    f.write(oauth_token_secret.as_bytes()).unwrap();

    Ok(())
}

#[derive(Debug)]
struct OauthTokens {
    oauth_token: String,
    oauth_token_secret: String,
}

impl OauthTokens {
    pub fn new(params: String) -> Self {
        let split: Vec<&str> = params.split("&").collect();

        match split.len() {
            3 | 4 => {},
            _ => panic!("Incorrect authorization payload size {:?}", split),
        }

        Self {
            oauth_token: split[0].replace("oauth_token=", ""),
            oauth_token_secret: split[1].replace("oauth_token_secret=", ""),
        }
    }
}

#[derive(Debug)]
struct CallbackParams {
    oauth_token: String,
    oauth_verifier: String,
}

impl CallbackParams {
    pub fn new(params: String) -> Self {
        let split: Vec<&str> = params.split("&").collect();

        match split.len() {
            2 => {},
            _ => panic!("error"),
        }

        Self {
            oauth_token: split[0].replace("oauth_token=", ""),
            oauth_verifier: split[1].replace("oauth_verifier=", ""),
        }
    }
}
*/
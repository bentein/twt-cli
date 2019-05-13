use crate::authenticate;
use crate::credentials;

use std::io::{Error, ErrorKind, Write, Read, BufRead};
use std::net::{TcpListener, TcpStream};
use std::path::PathBuf;
use std::fs::File;
use crate::credentials::{ApplicationCredentials, Credentials, UserCredentials};

pub fn authorize() -> std::io::Result<String> {

    let app_credentials: ApplicationCredentials = credentials::get_application_credentials()?;
    let credentials: Credentials = Credentials::new(app_credentials, UserCredentials::empty());
    let request_tokens: OauthTokens = get_request_token(&credentials)?;

    let username = prompt_user_name()?;

    webbrowser::open(&format!("https://api.twitter.com/oauth/authorize?oauth_token={}", &request_tokens.oauth_token))
        .expect("Couldn't open authorization page");

    let listener = TcpListener::bind("127.0.0.1:17800").unwrap();

    let callback_result: CallbackParams = match listener.accept() {
        Ok((socket, _addr)) => handle_callback(socket),
        Err(e) => panic!("Couldn't get client: {:?}", e),
    }?;

    let user_credentials: UserCredentials = get_user_credentials(username, &callback_result, &credentials)?;

    credentials::add_new_user_credentials(&user_credentials.name, &user_credentials.oauth_token, &user_credentials.oauth_token_secret)

}

fn get_request_token(credentials: &Credentials) -> std::io::Result<OauthTokens> {

    let client = reqwest::Client::new();

    let url: &str = "https://api.twitter.com/oauth/request_token";

    let headers = authenticate::get_authorization_header("post", url, [("oauth_consumer_key",credentials.app.application_key.as_str())].to_vec(), credentials)?;

    let res = client.post(url)
        .headers(headers)
        .send().unwrap()
        .text().unwrap();

    Ok(OauthTokens::new(res)?)
}

fn prompt_user_name() -> std::io::Result<String> {

    print!("Input a username to associate with this authorization: ");
    std::io::stdout().flush()?;

    let stdin = std::io::stdin();
    let uname = stdin.lock()
        .lines()
        .next()
        .expect("Invalid username input");

    uname
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

    Ok(params?)
}

fn get_user_credentials(name: String, tokens: &CallbackParams, credentials: &Credentials) -> std::io::Result<UserCredentials> {

    let oauth_consumer_key: &str = &credentials.app.application_key;

    let client = reqwest::Client::new();

    let res = client.post("https://api.twitter.com/oauth/access_token")
        .query(&[("oauth_consumer_key", oauth_consumer_key), ("oauth_token", &tokens.oauth_token), ("oauth_verifier", &tokens.oauth_verifier)])
        .send().unwrap()
        .text().unwrap();

    let split: Vec<&str> = res.split("&").collect();

    let token: String = split[0].replace("oauth_token=", "");
    let secret: String = split[1].replace("oauth_token_secret=", "");

    Ok(UserCredentials::new(name, token, secret))
}

#[derive(Debug)]
struct OauthTokens {
    oauth_token: String,
    oauth_token_secret: String,
}

impl OauthTokens {
    pub fn new(params: String) -> std::io::Result<Self> {
        let split: Vec<&str> = params.split("&").collect();

        match split.len() {
            3 | 4 => {},
            _ => return Err(Error::new(ErrorKind::InvalidData, format!("Incorrect authorization payload size {:?}", split))),
        }

        Ok(Self {
            oauth_token: split[0].replace("oauth_token=", ""),
            oauth_token_secret: split[1].replace("oauth_token_secret=", ""),
        })
    }
}



#[derive(Debug)]
struct CallbackParams {
    oauth_token: String,
    oauth_verifier: String,
}

impl CallbackParams {
    pub fn new(params: String) -> std::io::Result<Self> {
        let split: Vec<&str> = params.split("&").collect();

        match split.len() {
            2 => {},
            _ => return Err(Error::new(ErrorKind::PermissionDenied, format!("Authorization request denied"))),
        }

        Ok(Self {
            oauth_token: split[0].replace("oauth_token=", ""),
            oauth_verifier: split[1].replace("oauth_verifier=", ""),
        })
    }
}
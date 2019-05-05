use std::path::PathBuf;
use std::io::{Error, ErrorKind};

#[allow(dead_code)]
pub fn get_active_credentials() -> std::io::Result<Credentials> {

    match dirs::home_dir() {
        Some(path) => get_active_credentials_from_path(path),
        None => Err(Error::new(ErrorKind::NotFound, "Could not find your home directory")),
    }

}

fn get_active_credentials_from_path(_path: PathBuf) -> std::io::Result<Credentials> {

    let mut path = _path.clone();
    path.push(".twt");

    let mut config: String = std::fs::read_to_string(path)?;
    config = config.replace("\r", "");

    get_active_credentials_from_string(config)

}

fn get_active_credentials_from_string(config: String) -> std::io::Result<Credentials> {

    let app_credentials = get_application_credentials_from_string(config.to_string())?;
    let user_credentials = get_active_user_credentials_from_string(config.to_string())?;

    Ok(Credentials::new(app_credentials, user_credentials))
}


#[allow(dead_code)]
pub fn get_application_credentials() -> std::io::Result<ApplicationCredentials> {

    match dirs::home_dir() {
        Some(path) => get_application_credentials_from_path(path),
        None => Err(Error::new(ErrorKind::NotFound, "Could not find your home directory")),
    }

}

fn get_application_credentials_from_path(_path: PathBuf) -> std::io::Result<ApplicationCredentials> {

    let mut path = _path.clone();
    path.push(".twt");

    let mut config: String = std::fs::read_to_string(path)?;
    config = config.replace("\r", "");

    get_application_credentials_from_string(config)

}

fn get_application_credentials_from_string(config: String) -> std::io::Result<ApplicationCredentials> {

    let line_vec: Vec<&str> = config.split("\n").collect();

    let mut application_key: Option<&str> = None;
    let mut application_secret: Option<&str> = None;

    for i in 0..line_vec.len() {

        let split: Vec<&str> = line_vec[i].split(":").collect();
        if line_vec[i].contains("application_key:") {
            application_key = Some(split[1]);
        }
        else if line_vec[i].contains("application_secret:") {
            application_secret = Some(split[1]);
        }
    }

    match (application_key, application_secret) {
        (Some(application_key), Some(application_secret)) =>
            Ok(ApplicationCredentials::new(application_key.replace(" ", "").to_string(), application_secret.replace(" ", "").to_string())),
        (_,_) =>
            Err(Error::new(ErrorKind::NotFound, "No application credentials set"))
    }

}

#[allow(dead_code)]
pub fn get_active_user_credentials() -> std::io::Result<UserCredentials> {

    match dirs::home_dir() {
        Some(path) => get_active_user_credentials_from_path(path),
        None => Err(Error::new(ErrorKind::NotFound, "Could not find your home directory")),
    }

}

fn get_active_user_credentials_from_path(_path: PathBuf) -> std::io::Result<UserCredentials> {

    let mut path = _path.clone();
    path.push(".twt");

    let mut config: String = std::fs::read_to_string(path)?;
    config = config.replace("\r", "");

    get_active_user_credentials_from_string(config)

}

fn get_active_user_credentials_from_string(config: String) -> std::io::Result<UserCredentials> {

    let line_vec: Vec<&str> = config.split("\n").collect();

    let active_user: Option<&str> = get_active_user_name(line_vec);

    match active_user {
        Some(name) => get_user_credentials_from_string(name.to_string(), config),
        None => Err(Error::new(ErrorKind::NotFound, "No active user set"))
    }

}

fn get_active_user_name(line_vec: Vec<&str>) -> Option<&str> {

    for l in line_vec {

        if l.starts_with("active") {
            let line: Vec<&str> = l.split(":").collect();
            return Some(line[1].trim())
        }
    }

    None

}

#[allow(dead_code)]
pub fn get_user_credentials(name: String) -> std::io::Result<UserCredentials> {

    match dirs::home_dir() {
        Some(path) => get_user_credentials_from_path(name, path),
        None => Err(Error::new(ErrorKind::NotFound, "Could not find your home directory")),
    }

}

fn get_user_credentials_from_path(name: String, _path: PathBuf) -> std::io::Result<UserCredentials> {

    let mut path = _path.clone();
    path.push(".twt");

    let mut config: String = std::fs::read_to_string(path)?;
    config = config.replace("\r", "");

    get_user_credentials_from_string(name, config)

}

fn get_user_credentials_from_string(name: String, config: String) -> std::io::Result<UserCredentials> {

    let users: Vec<UserCredentials> = get_all_user_credentials_from_string(config)?;

    for c in users {
        if c.name == name {
            return Ok(c)
        }
    }

    Err(Error::new(ErrorKind::NotFound, format!("{}{}", "Could not find credentials for user ", name)))
}

#[allow(dead_code)]
pub fn get_all_user_credentials() -> std::io::Result<Vec<UserCredentials>> {

    match dirs::home_dir() {
        Some(path) => get_all_user_credentials_from_path(path),
        None => Err(Error::new(ErrorKind::NotFound, "Could not find your home directory")),
    }

}

fn get_all_user_credentials_from_path(_path: PathBuf) -> std::io::Result<Vec<UserCredentials>> {

    let mut path = _path.clone();
    path.push(".twt");

    let mut config: String = std::fs::read_to_string(path)?;
    config = config.replace("\r", "");

    get_all_user_credentials_from_string(config)

}

fn get_all_user_credentials_from_string(config: String) -> std::io::Result<Vec<UserCredentials>> {

    let line_vec: Vec<&str> = config.split("\n").collect();

    let mut credentials_vec: Vec<&str> = Vec::new();
    let mut users: Vec<UserCredentials> = Vec::new();

    for i in 0..line_vec.len() {

        if line_vec[i].contains("users:") {

            for mut j in i..line_vec.len() {

                if line_vec[j].contains("  -") {

                    credentials_vec.push(line_vec[j]);

                    j += 1;

                    while j < line_vec.len() && !line_vec[j].contains("  -") {
                        credentials_vec.push(line_vec[j]);
                        j += 1;
                    }

                    let credentials: Option<UserCredentials> = construct_user_credentials_from_vec(credentials_vec.clone());

                    match credentials {
                        Some(credentials) => { users.push(credentials); },
                        None => {}
                    }

                    credentials_vec.clear();
                    j -= 1;

                }

            }

            return Ok(users)
        }

    }

    Ok(users)
}

fn construct_user_credentials_from_vec(credentials_vec: Vec<&str>) -> Option<UserCredentials> {

    let mut name: Option<&str> = None;
    let mut oauth_token: Option<&str> = None;
    let mut oauth_token_secret: Option<&str> = None;

    for l in credentials_vec {

        let split: Vec<&str> = l.split(":").collect();
        if l.contains("name:") {
            name = Some(split[1]);
        }
        else if l.contains("oauth_token:") {
            oauth_token = Some(split[1]);
        }
        else if l.contains("oauth_token_secret:") {
            oauth_token_secret = Some(split[1]);
        }
    }

    match (name, oauth_token, oauth_token_secret) {
        (Some(name), Some(oauth_token), Some(oauth_token_secret)) =>
            Some(UserCredentials::new(name.replace(" ", "").to_string(), oauth_token.replace(" ", "").to_string(), oauth_token_secret.replace(" ", "").to_string())),
        (_,_,_) =>
            None
    }
}

#[derive(Clone, Debug)]
pub struct Credentials {
    pub app: ApplicationCredentials,
    pub user: UserCredentials,
}

impl Credentials {

    pub fn new(app: ApplicationCredentials, user: UserCredentials) -> Self {

        Self {
            app,
            user,
        }
    }
}

#[derive(Clone, Debug)]
pub struct ApplicationCredentials {
    pub application_key: String,
    pub application_secret: String,
}

impl ApplicationCredentials {

    pub fn new(application_key: String, application_secret: String) -> Self {

        Self {
            application_key,
            application_secret,
        }
    }
}

#[derive(Clone, Debug)]
pub struct UserCredentials {
    pub name: String,
    pub oauth_token: String,
    pub oauth_token_secret: String,
}

impl UserCredentials {

    pub fn new(name: String, oauth_token: String, oauth_token_secret: String) -> Self {

        Self {
            name,
            oauth_token,
            oauth_token_secret
        }
    }
}

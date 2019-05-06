use std::path::PathBuf;
use std::io::{Error, ErrorKind, Write};
use std::fs::File;

#[allow(dead_code)]
pub fn get_active_credentials() -> std::io::Result<Credentials> {

    match dirs::home_dir() {
        Some(path) => get_active_credentials_from_path(path),
        None => Err(Error::new(ErrorKind::NotFound, "Could not find your home directory")),
    }

}

fn get_active_credentials_from_path(_path: PathBuf) -> std::io::Result<Credentials> {

    let mut config: String = get_authorization_config_string(_path)
        .expect("No authorization configuration found");

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

    let mut config: String = get_authorization_config_string(_path)
        .expect("No authorization configuration found");

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
            Ok(ApplicationCredentials::new(application_key.replace(" ", ""), application_secret.replace(" ", ""))),
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

    let mut config: String = get_authorization_config_string(_path)
        .expect("No authorization configuration found");

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

        if l.contains("active_user:") {
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

    let mut config: String = get_authorization_config_string(_path)
        .expect("No authorization configuration found");

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

    let mut config: String = get_authorization_config_string(_path)
        .expect("No authorization configuration found");

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

pub fn set_application_credentials(application_key: &str, application_secret: &str) -> std::io::Result<String> {

    match dirs::home_dir() {
        Some(path) => {

            let _path = path.clone();
            let config = get_authorization_config_string(_path);

            match config {
                Ok(conf) => {
                    let existing_credentials: Vec<&str> = conf.split("\n").collect();
                    write_application_credentials(application_key, application_secret, existing_credentials, path)
                },
                Err(_e) => write_application_credentials(application_secret, application_secret, Vec::new(), path)
            }
        },
        None => Err(Error::new(ErrorKind::NotFound, "Could not find your home directory")),
    }

}

fn write_application_credentials(application_key: &str, application_secret: &str, existing_credentials: Vec<&str>, mut path: PathBuf) -> std::io::Result<String> {

    path.push(".twt");
    let mut existing_credentials: Vec<&str> = existing_credentials.clone();

    let mut key_write: bool = false;
    let mut secret_write: bool = false;

    let key = format!("{}: {}","application_key",application_key);
    let secret = format!("{}: {}","application_secret",application_secret);

    for i in 0..existing_credentials.len() {
        if existing_credentials[i].contains("application_key") {
            existing_credentials[i] = &key;
            key_write = true;
        }
        else if existing_credentials[i].contains("application_secret") {
            existing_credentials[i] = &secret;
            secret_write = true;
        }
    }

    if !key_write {
        existing_credentials.push(&key);
    }
    if !secret_write {
        existing_credentials.push(&secret);
    }

    let new_credentials: String = existing_credentials.join("\n");

    let mut f = File::create(path).unwrap();

    match f.write(new_credentials.as_bytes()) {
        Ok(_u) => Ok(format!("Application credentials set")),
        Err(e) => Err(e.into()),
    }

}

pub fn set_active_user(user: &str) -> std::io::Result<String> {

    match dirs::home_dir() {
        Some(path) => {

            let _path = path.clone();
            let config = get_authorization_config_string(_path);

            match config {
                Ok(conf) => {
                    let existing_credentials: Vec<&str> = conf.split("\n").collect();
                    write_active_user(user, existing_credentials, path)
                },
                Err(_e) => write_active_user(user, Vec::new(), path),
            }

        },
        None => Err(Error::new(ErrorKind::NotFound, "Could not find your home directory")),
    }

}

fn write_active_user(user: &str, existing_credentials: Vec<&str>, mut path: PathBuf) -> std::io::Result<String> {

    path.push(".twt");
    let mut existing_credentials: Vec<&str> = existing_credentials.clone();

    let mut name_write: bool = false;

    let usr = format!("{}: {}","active_user",user);

    for i in 0..existing_credentials.len() {
        if existing_credentials[i].contains("active_user:") {
            existing_credentials[i] = &usr;
            name_write = true;
        }
    }

    if !name_write {
        existing_credentials.push(&usr);
    }

    let new_credentials: String = existing_credentials.join("\n");

    let mut f = File::create(path).unwrap();

    match f.write(new_credentials.as_bytes()) {
        Ok(_u) => Ok(format!("User {} set as active", user)),
        Err(e) => Err(e.into()),
    }

}

pub fn add_new_user_credentials(name: &str, oauth_token: &str, oauth_token_secret: &str) -> std::io::Result<String> {

    let user: UserCredentials = UserCredentials::new(name.to_string(), oauth_token.to_string(), oauth_token_secret.to_string());

    match dirs::home_dir() {
        Some(path) => {

            let _path = path.clone();
            let config = get_authorization_config_string(_path);

            match config {
                Ok(conf) => write_new_user_credentials(user, conf, path),
                Err(_e) => write_new_user_credentials(user, String::new(), path),
            }

        },
        None => Err(Error::new(ErrorKind::NotFound, "Could not find your home directory")),
    }

}

fn write_new_user_credentials(user: UserCredentials, existing_credentials: String, path: PathBuf) -> std::io::Result<String> {

    let config: String = existing_credentials.clone();

    let mut all_users: Vec<UserCredentials> = get_all_user_credentials_from_string(config)?;

    for u in &all_users {
        if u.name == user.name {
            return Err(Error::new(ErrorKind::AlreadyExists, "User with that name already exists"))
        }
    }

    all_users.push(user.clone());

    match write_user_credentials_list(all_users, existing_credentials, path) {
        Ok(()) => Ok(format!("User {} added", user.name)),
        Err(e) => Err(e.into()),
    }

}

pub fn delete_user_credentials(user_name: &str) -> std::io::Result<String> {

    match dirs::home_dir() {
        Some(path) => {

            let _path = path.clone();
            let config = get_authorization_config_string(_path);

            match config {
                Ok(conf) => do_delete_user_credentials(user_name, conf, path),
                Err(_e) => Err(Error::new(ErrorKind::NotFound, format!("Could not find your credentials config at {:?}", path))),
            }

        },
        None => Err(Error::new(ErrorKind::NotFound, "Could not find your home directory")),
    }

}

fn do_delete_user_credentials(user_name: &str, existing_credentials: String, path: PathBuf) -> std::io::Result<String> {

    let config: String = existing_credentials.clone();

    let mut all_users: Vec<UserCredentials> = get_all_user_credentials_from_string(config)?;
    let len_before = all_users.len();

    all_users.retain(|u| u.name != user_name);
    let len_after = all_users.len();

    if len_before == len_after {
        return Err(Error::new(ErrorKind::NotFound, format!("No user with name {} found", user_name)))

    }

    match write_user_credentials_list(all_users, existing_credentials, path) {
        Ok(()) => Ok(format!("User {} deleted", user_name)),
        Err(e) => Err(e.into()),
    }

}

fn write_user_credentials_list(users: Vec<UserCredentials>, existing_credentials: String, mut path: PathBuf) -> std::io::Result<()> {

    path.push(".twt");

    let existing: Vec<&str> = existing_credentials.split("\n").collect();
    let mut credentials: Vec<&str> = Vec::new();
    for i in 0..existing.len() {
        if existing[i].contains("users:") {
            credentials = existing[0..(i+1)].to_vec();
        }
    }

    let mut user_strings: Vec<String> = Vec::new();
    for user in &users {
        let u: String = format!("  - name: {}\n    oauth_token: {}\n    oauth_token_secret: {}", user.name, user.oauth_token, user.oauth_token_secret);
        user_strings.push(u);
    }

    for user in &user_strings {
        credentials.push(user);
    }

    let new_credentials: String = credentials.join("\n");

    let mut f = File::create(path).unwrap();

    match f.write(new_credentials.as_bytes()) {
        Ok(_u) => Ok(()),
        Err(e) => Err(e.into())
    }

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

fn get_authorization_config_string(_path: PathBuf) -> std::io::Result<String> {

    let mut path = _path.clone();
    path.push(".twt");

    std::fs::read_to_string(path)
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

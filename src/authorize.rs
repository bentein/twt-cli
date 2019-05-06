use crate::credentials;

use std::io::{Error, ErrorKind};

pub fn authorize_user(delete: &bool, active: &bool, name: &str, token: &Option<String>, secret: &Option<String>) -> std::io::Result<String> {

    match (delete, active) {
        (false, false) => {
            match (token, secret) {
                (Some(token), Some(secret)) => credentials::add_new_user_credentials(name, token, secret),
                (_,_) => Err(Error::new(ErrorKind::InvalidInput, "Missing arguments")),
            }
        },
        (true, false) => credentials::delete_user_credentials(name),
        (false, true) => credentials::set_active_user(name),
        (_,_) => Err(Error::new(ErrorKind::InvalidInput, "Multiple flags cannot be input to 'authorize user' at once")),
    }

}

pub fn authorize_app(token: &str, secret: &str) -> std::io::Result<String> {

    credentials::set_application_credentials(token, secret)

}
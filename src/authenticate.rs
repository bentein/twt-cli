use std::iter;
use std::time::{SystemTime, UNIX_EPOCH, Duration};

use rand::{Rng, thread_rng};
use rand::distributions::Alphanumeric;

use crypto::hmac::Hmac;
use crypto::mac::{Mac, MacResult};
use crypto::sha1::Sha1;

use reqwest::header::{HeaderMap, HeaderValue, AUTHORIZATION};

pub fn get_authorization_header(method: &str, base_url: &str, _params: Vec<(&str,&str)>) -> std::io::Result<HeaderMap> {

    let mut params: Vec<(&str,&str)> = _params.clone();
    let mut header_map = HeaderMap::new();

    let nonce: String = generate_nonce();
    let duration: Duration = SystemTime::now().duration_since(UNIX_EPOCH).unwrap();
    let timestamp: String = duration.as_secs().to_string();

    params.push(("oauth_nonce", nonce.as_str()));
    params.push(("oauth_signature_method", "HMAC-SHA1"));
    params.push(("oauth_timestamp", timestamp.as_str()));
    params.push(("oauth_version", "1.0"));

    let auth: Vec<(&str,&str)> = params.clone();

    let signature: String = get_oauth_signature(method, base_url, auth)?;

    params.push(("oauth_signature", signature.as_str()));

    let mut header: String = String::new();
    header.push_str("OAuth ");

    for p in params {
        if p.0.starts_with("oauth_") {
            let param: String = format!("{}=\"{}\"", p.0, p.1);
            header.push_str(&param);
            header.push_str(",");
        }
    }

    header.truncate(header.len() - 1);

    header_map.insert(AUTHORIZATION, HeaderValue::from_str(&header).unwrap());
    Ok(header_map)
}

pub fn get_oauth_signature(_method: &str, _base_url: &str, _params: Vec<(&str,&str)>) -> std::io::Result<String> {

    let params: Vec<(&str, &str)> = _params.clone();
    let mut key_tuple: (&str, &str) = ("", "");

    for p in params {
        if p.0 == "oauth_consumer_key" {
            key_tuple.0 = "IMC0fpDcLgmTBmTuWrwzX4BciLDPExAjMmqhiDM5JagzuqSp1X";
        }
        if p.0 == "oauth_token" {
            key_tuple.1 = "noxb9bdwPDXg9FfRzPxgtgkcEAC2Is3U6oxzXjFsgdn2t";
        }
    }

    let key: &str = &format!("{}&{}", key_tuple.0, key_tuple.1);

    let method: String = _method.to_uppercase();
    let base_url: String = percent_encode(_base_url)?;
    let parameters: String = get_parameter_string(_params)?;

    let signature: String = format!("{}&{}&{}", method, base_url, parameters);

    let hmac_encoded: String = hmac_sha1(&signature, key)?;
    let percent_encode: String = percent_encode(&hmac_encoded)?;

    Ok(percent_encode)
}

fn get_parameter_string(mut params: Vec<(&str,&str)>) -> std::io::Result<String> {

    let mut dst: String = String::new();

    params.sort();

    for p in params {
        let param = format!("{}={}", p.0, p.1);
        dst.push_str(&param);
        dst.push_str("&");
    }

    dst.truncate(dst.len() - 1);
    dst = percent_encode(&dst)?;

    Ok(dst)
}

fn percent_encode(src: &str) -> std::io::Result<String> {

    let reserved: [u8; 66] = [
        0x30, 0x31, 0x32, 0x33, 0x34, 0x35, 0x36, 0x37, 0x38, 0x39,
        0x41, 0x42, 0x43, 0x44, 0x45, 0x46, 0x47, 0x48, 0x49, 0x4A, 0x4B,0x4C, 0x4D, 0x4E, 0x4F,
        0x50, 0x51, 0x52, 0x53, 0x54, 0x55, 0x56, 0x57, 0x58, 0x59, 0x5A,
        0x61, 0x62, 0x63, 0x64, 0x65, 0x66, 0x67, 0x68, 0x69, 0x6A, 0x6B,0x6C, 0x6D, 0x6E, 0x6F,
        0x70, 0x71, 0x72, 0x73, 0x74, 0x75, 0x76, 0x77, 0x78, 0x79, 0x7A,
        0x2D, 0x2E, 0x5F, 0x7E,
    ];

    let mut dst: String = String::new();
    let bytes: &[u8] = src.as_bytes();

    for b in bytes {
        if reserved.contains(b) {

            dst.push(*b as char);

        } else {

            dst.push_str("%");

            let hex = format!("{:X?}", b);
            dst.push_str(&hex);
        }
    }

    Ok(dst)
}

fn hmac_sha1(_message: &str, _key: &str) -> std::io::Result<String> {
    let key: &[u8] = _key.as_bytes();
    let message: &[u8] = _message.as_bytes();

    let mut mac: Hmac<Sha1> = Hmac::new(Sha1::new(), key);
    mac.input(message);

    let mac_result: MacResult = mac.result();
    let result: String = base64::encode(mac_result.code());

    Ok(result)
}


fn generate_nonce() -> String {
    
    let mut rng = thread_rng();
    let chars: String = iter::repeat(())
            .map(|()| rng.sample(Alphanumeric))
            .take(42)
            .collect();
    
    chars
}
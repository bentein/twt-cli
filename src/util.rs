
pub fn get_full_request_url(base_url: &str, req_params: Vec<(&str,&str)>) -> std::io::Result<String> {

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

pub fn unwrap_parameter<'l>(name: &'l str, _param: &'l Option<String>) -> (&'l str, &'l str) {

    match &_param {
        Some(p) => (name, p),
        None => ("",""),
    }

}
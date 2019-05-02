use crate::authenticate;

pub fn get_timeline(user: &Option<String>) {

    match user {
        Some(username) => { get_user_timeline(username).unwrap(); },
        None => { get_own_timeline().unwrap(); },
    }
}

fn get_user_timeline(username: &str) -> std::io::Result<()> {

    let client = reqwest::Client::new();

    let user = ("screen_name", username);

    let base_url = "https://api.twitter.com/1.1/statuses/user_timeline.json";
    let params: Vec<(&str,&str)> = [("oauth_consumer_key","NmnoD1Pew3ho5ZoHITn1JjaLw"),("oauth_token","31216914-ta4Vira4eL8dUl59WH0Q3zPhBiYcc05DgSuaQEGVN"),user].to_vec();

    let headers = authenticate::get_authorization_header("get", base_url, params)?;
    let url = format!("{}?{}={}", base_url, user.0, user.1);

    let res = client.get(&url)
        .headers(headers)
        .send().unwrap()
        .text();

    println!("{:?}", res);

    Ok(())

}

fn get_own_timeline() -> std::io::Result<()> {

    let client = reqwest::Client::new();
    
    let base_url = "https://api.twitter.com/1.1/statuses/home_timeline.json";
    let params: Vec<(&str,&str)> = [("oauth_consumer_key","NmnoD1Pew3ho5ZoHITn1JjaLw"),("oauth_token","31216914-ta4Vira4eL8dUl59WH0Q3zPhBiYcc05DgSuaQEGVN")].to_vec();

    let headers = authenticate::get_authorization_header("get", base_url, params)?;

    println!("{:?}", headers);

    let res = client.get(base_url)
        .headers(headers)
        .send().unwrap()
        .text();
    
    println!("{:?}", res);

    Ok(())
}

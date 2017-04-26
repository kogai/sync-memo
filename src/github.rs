use std::env;
use std::io::Read;
use dotenv;
use reqwest::Client;
use hyper::header::{Authorization};

const GITHUB_API: &'static str = "https://api.github.com";

fn authorization() -> Authorization<String> {
    let path_to_env = env::home_dir().and_then(|a| Some(a.join("sync-memo").join(".env")));
    match path_to_env {
        Some(x) => {
            dotenv::from_path(x.as_path()).ok();
        },
        None => {},
    };
    let access_token = env::var("GITHUB_ACCESS_TOKEN").expect(&format!("Missing environment parameter GITHUB_ACCESS_TOKEN"));
    Authorization(format!("token {}", access_token))
}

pub fn get_own_gists(user_name: &str) -> String {
    let http_client = Client::new().expect("Create HTTP client is failed");
    let url = format!("{}/users/{}/gists", GITHUB_API, user_name);
    let mut buffer = String::new();

    http_client
        .get(url.as_str())
        .header(authorization())
        .send()
        .expect("send Request failed")
        .read_to_string(&mut buffer)
        .expect("read response failed");

    println!("{}", buffer);
    url
}
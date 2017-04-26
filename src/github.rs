use std::env;
use std::collections::HashMap;

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

#[derive(Debug, Serialize, Deserialize)]
struct GistFile {
    size: i32,
    language: String,
    content: String,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct GistFiles {
    files: HashMap<String, GistFile>,
}

#[derive(Debug, Serialize, Deserialize)]
struct Content {
    content: String,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct CreateGist {
    description: String,
    public: bool,
    files: HashMap<String, Content>,
}

pub fn create_gist() -> GistFiles {
    let http_client = Client::new().expect("Create HTTP client is failed");
    let url = format!("{}/gists", GITHUB_API);
    let mut files = HashMap::new();
    files.insert("test.md".to_owned(), Content {
        content: "test body.".to_owned(),
    });
    let request_body = CreateGist {
        description: "this is test".to_owned(),
        public: false,
        files: files,
    };

    http_client
        .post(url.as_str())
        .header(authorization())
        .json(&request_body)
        .send()
        .expect("send Request failed")
        .json::<GistFiles>()
        .expect("read response failed")
}

pub fn get_gist(gist_id: &str) -> GistFiles {
    let http_client = Client::new().expect("Create HTTP client is failed");
    let url = format!("{}/gists/{}", GITHUB_API, gist_id);

    http_client
        .get(url.as_str())
        .header(authorization())
        .send()
        .expect("send Request failed")
        .json::<GistFiles>()
        .expect("read response failed")
}

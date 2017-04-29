use std::env;
use std::collections::HashMap;
use std::path::Path;

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
pub struct GistFile {
    size: i32,
    language: String,
    content: String,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct GistFiles {
    pub id: String,
    pub files: HashMap<String, GistFile>,
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

fn path_to_file_name(path_to_file: String) -> String {
    let file_name = Path::new(&path_to_file).file_name();
    match file_name {
        Some(n) => {
            n.to_str().unwrap_or("").to_string()
        },
        None => "".to_string(),
    }
}

#[test]
fn it_should_convert_path_to_file_name() {
    assert_eq!(path_to_file_name("/foo/bar/buzz.text".to_owned()), "buzz.text".to_owned());
}

pub fn create_gist(path_to_file: String, content: String) -> GistFiles {
    let http_client = Client::new().expect("Create HTTP client is failed");
    let url = format!("{}/gists", GITHUB_API);
    let mut files = HashMap::new();
    let file_name_string = path_to_file_name(path_to_file);

    files.insert(file_name_string, Content { content });
    let request_body = CreateGist {
        description: "".to_owned(),
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

pub fn modify_gist(gist_id: &str, file_name: String, contents: String) -> GistFiles {
    let http_client = Client::new().expect("Create HTTP client is failed");
    let url = format!("{}/gists/{}", GITHUB_API, gist_id);
    let mut files = HashMap::new();
    files.insert(path_to_file_name(file_name), Content {
        content: contents,
    });
    let request_body = CreateGist {
        description: "this is update test".to_owned(),
        public: false,
        files: files,
    };

    http_client
        .patch(url.as_str())
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

#[macro_use]
extern crate serde_derive;

extern crate dotenv;
extern crate reqwest;
extern crate serde;
extern crate serde_json;
extern crate clap;
extern crate config_file_handler;
extern crate hyper;

mod github;

fn main() {
    let gists = github::get_gist("5c48d55cac77922fb1dd2162e48256f7");
    let gist_created = github::create_gist();
    println!("{:?}", gists);
    println!("{:?}", gist_created);
} 
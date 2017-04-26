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
    let gists = github::get_own_gists("kogai");
    println!("{:?}", gists);
} 
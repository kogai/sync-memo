use std::path::{PathBuf};
use std::fs::File;
use std::io::{Read, Write};
use std::sync::mpsc::channel;

use crossbeam;
use serde_json;
use watcher;
use github;

#[derive(Debug, Serialize, Deserialize)]
pub struct WatchFile {
    gist_id: String,
    file_path: String,
}

#[derive(Debug)]
pub struct FileHandler {
    path_to_setting: String,
    files: Vec<WatchFile>,
}

impl FileHandler {
    pub fn new(path_to_setting: PathBuf) -> Self {
        let mut buffer = String::new();
        let mut file = File::open(&path_to_setting).unwrap();
        file.read_to_string(&mut buffer).unwrap();
        let files = serde_json::from_str::<Vec<WatchFile>>(&buffer).unwrap();
        FileHandler {
            path_to_setting: path_to_setting.to_str().unwrap().to_owned(),
            files,
        }
    }

    pub fn watch_all_files<'a>(&'a self) -> Vec<crossbeam::ScopedJoinHandle<()>>{
        crossbeam::scope(|scope| {
          (&self.files)
            .iter()
            .map(|file| {
                scope.spawn(move || {
                    watcher::watch(file.file_path.to_owned(), &file.gist_id, channel());
                })
            })
            .collect::<Vec<_>>()
        })
    }

    pub fn add_files(&self, file_path: String) {
        let mut buffer = String::new();
        let mut file = File::open(&file_path).unwrap();
        file.read_to_string(&mut buffer).unwrap();
        let result = github::create_gist(file_path.clone(), buffer);
        
        let mut setting_buffer = String::new();
        let mut setting_file = File::open(&self.path_to_setting).unwrap();
        let mut saved_files = serde_json::from_str::<Vec<WatchFile>>(&mut setting_buffer).unwrap();
        saved_files.push(WatchFile {
            gist_id: result.id,
            file_path,
        });

        setting_file.write_all(
            serde_json::to_string_pretty(&saved_files)
                .unwrap()
                .as_bytes()
        ).unwrap();
    }
}

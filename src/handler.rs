use std::path::PathBuf;
use std::fs::File;
use std::io::{Read, Write};
use std::sync::mpsc::channel;
use std::thread::{spawn, JoinHandle};

use serde_json;
use watcher;
use github;

#[derive(Debug, Serialize, Deserialize, Clone)]
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
            files: files,
        }
    }

    // TODO: Perhaps is should return JoinHandle<Channel> to enable to send message
    pub fn watch_all_files(&self) -> Vec<JoinHandle<()>> {
        (&self.files)
            .iter()
            .map(|file| self.watch_file(file.clone()))
            .collect()
    }

    pub fn watch_file(&self, add_file: WatchFile) -> JoinHandle<()> {
        spawn(move || {
            watcher::watch(add_file.file_path.to_owned(), &add_file.gist_id, channel());
        })
    }

    pub fn add_file(&self, file_path: String) -> WatchFile {
        let mut buffer = String::new();
        let mut file = File::open(&file_path).unwrap();
        file.read_to_string(&mut buffer).unwrap();
        let result = github::create_gist(file_path.clone(), buffer);
        let add_file = WatchFile {
            gist_id: result.id,
            file_path: file_path,
        };

        let mut setting_buffer = String::new();
        let mut setting_file = File::open(&self.path_to_setting).unwrap();
        setting_file.read_to_string(&mut setting_buffer).unwrap();
        let mut saved_files = serde_json::from_str::<Vec<WatchFile>>(&setting_buffer).unwrap();

        saved_files.push(add_file.clone());

        File::create(&self.path_to_setting)
            .unwrap()
            .write_all(serde_json::to_string_pretty(&saved_files)
                .unwrap()
                .as_bytes())
            .unwrap();

        add_file
    }

    pub fn get_file_ids(&self) -> Vec<String> {
        self.files.iter().map(|f| f.gist_id.to_owned()).collect()
    }
}

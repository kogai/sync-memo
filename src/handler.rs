use std::thread;
use std::path::{PathBuf, Path};
use std::fs::File;
use std::io::Read;
use serde_json;
use watcher;

#[derive(Debug)]
pub struct FileHandler {
    files: Vec<WatchFile>,
}

impl FileHandler {
    pub fn new(path_to_setting: PathBuf) -> Self {
        let mut buffer = String::new();
        let mut file = File::open(path_to_setting).unwrap();
        file.read_to_string(&mut buffer).unwrap();
        let files = serde_json::from_str::<Vec<WatchFile>>(&buffer).unwrap();
        FileHandler {
            files,
        }
    }

    pub fn watch(&self) {
        for f in &self.files {
            // thread::spawn(move || {
                // let file_path = Path::new(&f.file_path);
            watcher::watch(Path::new(&f.file_path).to_path_buf(), &f.gist_id);
            // });
        };
    }
}

#[derive(Debug, Serialize, Deserialize)]
pub struct WatchFile {
    gist_id: String,
    file_path: String,
}


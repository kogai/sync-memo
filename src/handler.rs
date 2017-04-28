use std::path::{PathBuf, Path};
use std::fs::File;
use std::io::Read;
use crossbeam;
use serde_json;
use watcher;

#[derive(Debug, Serialize, Deserialize)]
pub struct WatchFile {
    gist_id: String,
    file_path: String,
}

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

    pub fn watch<'a>(&'a self) -> Vec<crossbeam::ScopedJoinHandle<()>>{
        crossbeam::scope(|scope| {
          (&self.files)
            .iter()
            .map(|file| {
                println!("spaen: {:?}", file);
                scope.spawn(move || {
                    watcher::watch(Path::new(&file.file_path).to_path_buf(), &file.gist_id);
                })
            })
            .collect::<Vec<_>>()
        })
    }
}

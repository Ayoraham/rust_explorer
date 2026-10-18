use crate::utils;
use chrono::{DateTime, Local};
use homedir::my_home;
use std::{
    env,
    fmt::Display,
    fs::{self, Metadata},
    path::{Path, PathBuf},
};

pub struct Env {
    pub current_location: PathBuf,
    pub meta_data: Metadata,
    pub folders: Vec<String>,
    pub files: Vec<String>,
}

fn process_directory(current_dir: &Path) -> Result<(Vec<String>, Vec<String>), std::io::Error> {
    let mut files = Vec::new();
    let mut folders = Vec::new();
    let dir_contents = std::fs::read_dir(current_dir)?;
    for dir in dir_contents {
        let dir = match dir {
            Ok(val) => val,
            Err(_) => continue,
        };
        let filename = dir.file_name().to_string_lossy().into_owned();
        if dir.file_type()?.is_file() {
            files.push(filename);
        } else {
            folders.push(filename);
        }
    }
    Ok((folders, files))
}

impl Env {
    pub fn new() -> Self {
        println!("Welcome to rust Explorer\nEnter --help for more info");
        let mut start_args = env::args().skip(1).take(1);
        let start_path = match start_args.next() {
            Some(val) => PathBuf::from(val),
            None => {
                let start_path = match my_home().unwrap() {
                    Some(t) => t,
                    None => {
                        println!(
                            "Could not locate System Root Dir, Starting from current directory"
                        );
                        env::current_dir().unwrap()
                    }
                };
                start_path
            }
        };

        env::set_current_dir(&start_path)
            .expect("Could not navigate to root directory, Crahsing... :)");
        let (folders, files) = process_directory(&start_path).unwrap();
        Self {
            meta_data: fs::metadata(&start_path).unwrap(),
            folders,
            files,
            current_location: start_path,
        }
    }

    fn update(&mut self, new_pth: PathBuf) -> Result<(), std::io::Error> {
        let new_meta_data = new_pth.metadata()?;
        let (new_folders, new_files) = process_directory(&new_pth)?;
        env::set_current_dir(&new_pth)?;
        self.meta_data = new_meta_data;
        self.files = new_files;
        self.folders = new_folders;
        self.current_location = new_pth;
        Ok(())
    }

    pub fn view_dir_content_flagged(&self, flag: &str) {
        match flag {
            "folder" => {
                for dir in &self.folders {
                    println!("{dir}")
                }
            }
            "file" => {
                for dir in &self.files {
                    println!("{dir}")
                }
            }
            _ => println!("Invalid flag parameter"),
        }
    }

    pub fn view_dir_content(&self) {
        let dir_contents = std::fs::read_dir(&self.current_location).unwrap();
        for dir in dir_contents {
            println!("{}", dir.unwrap().file_name().into_string().unwrap())
        }
        utils::print_current_dir(&self.current_location);
    }

    pub fn navigate(&mut self, flag: &str) {
        let new_pth = match flag {
            ".." => {
                let mut new_pth = self.current_location.clone();
                let removed = new_pth.pop();
                if removed {
                    Some(new_pth)
                } else {
                    eprintln!("Cannot go back");
                    None
                }
            }
            dir => {
                let mut new_pth = self.current_location.clone();
                new_pth.push(dir);
                Some(new_pth)
            }
        };
        if let Some(pth) = new_pth{
            match self.update(pth){
                Ok(()) => (),
                Err(msg) => eprintln!("There was an error: {msg}")
            }
        }
    }
    pub fn goto(&mut self, abs_pth: &str) {
        let abs_pth = PathBuf::from(abs_pth);
        match self.update(abs_pth){
            Ok(()) => (),
            Err(msg) => eprintln!("There was an error: {msg}")
        }
    }

    pub fn dir_overview(&self) {
        // No of files
        // No folders
        // Size of dir
        // Date created
        // Last Edited
        println!("Number of files: {:?}", self.files.len());
        println!("Number of folders: {:?}", self.folders.len());

        let created: DateTime<Local> = self.meta_data.created().unwrap().into();
        println!("Date created: {}", created.format("%Y-%m-%d %H:%M"));
        let edited: DateTime<Local> = self.meta_data.modified().unwrap().into();
        println!("Last edited: {}", edited.format("%Y-%m-%d %H:%M"));

        //Name of folder
        println!(
            "Name of folder: {}",
            self.current_location.file_name().unwrap().display()
        );
        println!("{}", get_size(&self.current_location));
    }

}

#[derive(Debug)]
struct Size {
    bytes: u64,
    skipped_entries: u64,
    skipped_folders: u64,
}

impl Size {
    fn new() -> Self {
        Self {
            bytes: 0,
            skipped_entries: 0,
            skipped_folders: 0,
        }
    }
}

impl Display for Size {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "Size of folder {}\nSkipped Folders {}\nSkipped Entries {}",
            utils::format_size(self.bytes),
            self.skipped_folders,
            self.skipped_entries
        )
    }
}

fn get_size(path: &Path) -> Size {
    let mut size = Size::new();
    let entries = match path.read_dir() {
        Ok(val) => val,
        Err(_) => {
            size.skipped_folders += 1;
            return size;
        }
    };
    for entry in entries {
        let direntry = match entry {
            Ok(val) => val,
            Err(_) => {
                size.skipped_entries += 1;
                continue;
            }
        };
        let metadata = match direntry.metadata() {
            Ok(val) => val,
            Err(_) => {
                size.skipped_entries += 1;
                continue;
            }
        };
        if metadata.is_dir() {
            let sub = get_size(&direntry.path());
            size.bytes += sub.bytes;
            size.skipped_entries += sub.skipped_entries;
            size.skipped_folders += sub.skipped_folders;
        } else {
            size.bytes += metadata.len()
        }
    }

    size
}

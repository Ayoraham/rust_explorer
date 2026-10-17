use crate::utils;
use chrono::{DateTime, Local};
use homedir::my_home;
use std::{
    env,
    fs::{self, Metadata},
    path::{Path, PathBuf},
};

pub struct Env {
    pub current_location: PathBuf,
    pub meta_data: Metadata,
    pub folders: Vec<String>,
    pub files: Vec<String>,
}

fn get_dirs(current_dir: &PathBuf) -> (Vec<String>, Vec<String>) {
    let mut files = Vec::new();
    let mut folders = Vec::new();
    let dir_contents = std::fs::read_dir(current_dir).expect("Unable to iterate through dirctory");
    for dir in dir_contents {
        let dir = dir.expect("Could not get file path");
        let filename = dir.file_name().into_string().unwrap();
        if dir.file_type().unwrap().is_file() {
            files.push(filename);
        } else {
            folders.push(filename);
        }
    }
    (folders, files)
}

impl Env {
    pub fn new() -> Self {
        println!("Welcome to rust Explorer\nEnter --help for more info");
        let start_path = match my_home().unwrap() {
            Some(t) => t,
            None => {
                println!("Could not locate System Root Dir, Starting from current directory");
                env::current_dir().unwrap()
            }
        };
        env::set_current_dir(&start_path)
            .expect("Could not navigate to root directory, Crahsing... :)");
        Self {
            meta_data: fs::metadata(&start_path).unwrap(),
            folders: get_dirs(&start_path).0,
            files: get_dirs(&start_path).1,
            current_location: start_path,
        }
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

    pub fn move_to_dir(path: &Path) {
        std::env::set_current_dir(path).unwrap();
        utils::print_current_dir(path);
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

        println!("Size of folder: {:.2}Mb", self.get_dir_size() / 1024.0 / 1024.0);
    }
}

impl Env {
    fn get_dir_size(&self) -> f64 {
        let mut total_size: f64 = 0.0;
        for file in &self.files {
            let file_loc = self.current_location.join(file);
            total_size += file_loc
                .metadata()
                .expect("Could not get file metadate")
                .len() as f64;
        }
        total_size
    }
    
}

fn get_size(path:&Path) -> u64{
    let mut size = 0;
    for file in path.read_dir().unwrap().flatten(){
        if file.metadata().unwrap().is_dir(){
            size += get_size(&file.path());
        }else{
            size += file.metadata().unwrap().len();
        }

    }
    size
    }

 
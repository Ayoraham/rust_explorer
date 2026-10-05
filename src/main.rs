use colored::Colorize;
use homedir::my_home;
use std::{env, fs, io::stdin, path::{PathBuf, Path}};

fn get_user_input() -> String {
    let mut user_input = String::new();
    stdin()
        .read_line(&mut user_input)
        .expect("Failed to read user input");
    user_input.trim().to_string()
}

struct Env {
    current_location: PathBuf,
}

impl Env {
    fn new() -> Self {
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
            current_location: start_path,
        }
    }

    fn view_dir_content(self) {
        let dir_contents = std::fs::read_dir(self.current_location).unwrap();
        for dir in dir_contents {
            println!("{:?}", dir.unwrap())
        }
    }

    fn move_to_dir(path: &Path) {
        std::env::set_current_dir(path).unwrap();
    }
}

fn print_current_dir(current_dir: &PathBuf) {
    let current_dir = current_dir.to_str().unwrap();
    print!("{}>", current_dir.green())
}

fn main() {}

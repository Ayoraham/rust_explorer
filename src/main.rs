use colored::Colorize;
use homedir::my_home;
use std::{env, fmt::format, fs, io::stdin, path::{Path, PathBuf}};

const  INPUT_VARIATIONS: [&str; 1] = ["listdir"];

fn get_user_input() -> Vec<String> {
    let mut user_input = String::new();
    stdin()
        .read_line(&mut user_input)
        .expect("Failed to read user input");
    user_input.trim().to_string();
    let split_input: Vec<String> = user_input.splitn(2, " ").map(|word| word.to_string()).collect();
    split_input
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

    fn view_dir_content(&self) {
        let dir_contents = std::fs::read_dir(&self.current_location).unwrap();
        for dir in dir_contents {
            println!("{:?}", dir.unwrap())
        };
        print_current_dir(&self.current_location);
    }

    fn move_to_dir(path: &Path) {
        std::env::set_current_dir(path).unwrap();
        print_current_dir(path);
    }
}

fn print_current_dir(current_dir: impl AsRef<Path>) {
    let text = format!("Rust-Explorer  {}>", current_dir.as_ref().display().to_string());
    println!("{}",text.green())
}

fn main() {
    loop {
        let user_env = Env::new();
        print_current_dir(user_env.current_location);
        let user_input: Vec<String> = get_user_input();
        let parsed_input: Vec<&str> = user_input.iter().map(|word| word.as_str()).collect();
        let parsed_input = parsed_input.as_slice();
        
        match parsed_input {
            ["listdir"] => println!("Calling List dir..."),
            _ => println!("Function call not recognized")
        }
        
        println!("{:?}", parsed_input);
        //match user_input{
        //    "goto" =>
        //}
        
        //println!("{:?}", user_input)

        
    }

}

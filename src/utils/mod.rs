use colored::Colorize;
use std::io::stdin;
use std::path::Path;

pub fn get_user_input() -> Vec<String> {
    let mut user_input = String::new();
    stdin()
        .read_line(&mut user_input)
        .expect("Failed to read user input");
    user_input = user_input.trim().to_string();
    let split_input: Vec<String> = user_input
        .splitn(2, " ")
        .map(|word| word.to_string())
        .collect();
    split_input
}

pub fn print_current_dir(current_dir: impl AsRef<Path>) {
    let text = format!(
        "Rust-Explorer  {}>",
        current_dir.as_ref().display().to_string()
    );
    println!("{}", text.green())
}

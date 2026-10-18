use rust_explorer::{env, system, utils};
use std::{
    fs::{self, Metadata},
    path::{Path, PathBuf},
};
use walkdir::WalkDir;

//move_to_dir
//.move
//handle all results, instead of unwraps
// add a cd cmd to move to a nearby loc

fn main() {
    let mut user_env = env::Env::new();

    loop {
        utils::print_current_dir(&user_env.current_location);
        let user_input: Vec<String> = utils::get_user_input();
        let parsed_input: Vec<&str> = user_input.iter().map(|word| word.as_str()).collect();
        let parsed_input = parsed_input.as_slice();

        match parsed_input {
            ["sysinfo"] => system::get_sysinfo(),
            ["listdir"] => user_env.view_dir_content(),
            ["listdir", flag] => user_env.view_dir_content_flagged(flag),
            ["dir_overview"] => user_env.dir_overview(),
            ["cd", flag] => user_env.navigate(*flag),
            //["size"] =>

            //["listdir", flag] => match flag {
            // "folder"   => ,
            // "file" => ,
            // _ => println!("Unrecognized Flag")
            //},
            _ => println!("Function call not recognized"),
        }

        println!("{:?}", user_input);
    }
}

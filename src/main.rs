use rust_explorer::{env, system, utils};
use walkdir::WalkDir;
use std::{
    fs::{self, Metadata},
    path::{Path, PathBuf},
};

//move_to_dir
//.move
//handle all results, instead of unwraps
// add a cd cmd to move to a nearby loc

fn main() {
    loop {
        let user_env = env::Env::new();
        utils::print_current_dir(&user_env.current_location);
        let user_input: Vec<String> = utils::get_user_input();
        let parsed_input: Vec<&str> = user_input.iter().map(|word| word.as_str()).collect();
        let parsed_input = parsed_input.as_slice();

        match parsed_input {
            ["sysinfo"] => system::get_sysinfo(),
            ["listdir"] => user_env.view_dir_content(),
            ["listdir", flag] => user_env.view_dir_content_flagged(flag),
            ["dir_overview"] => user_env.dir_overview(),
            ["size"] => println!("{}", get_size(Path::new("C:/Users/Hp/Desktop/rustworks/rust_explorer"))),

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
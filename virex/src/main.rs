use std::{
    env::args,
    io::{Write, stdin, stdout},
};

mod config;
mod help;
mod info;
mod install;
mod uninstall;
mod update;

fn main() {
    let args: Vec<String> = args().collect(); // Collects all arguments

    // Skips name of package manager and checks all arguments
    let mut iter = args.iter().skip(1).peekable();

    while let Some(arg) = iter.next() {
        match arg.as_str() {
            "install" => {
                if config::exists() && config::has_source() {
                    // install function here..
                } else {
                    println!("config.toml is not setup correctly. Create new with 'virex config'.")
                }
            }
            "update" => {
                if config::exists() && config::has_source() {
                    // update function here...
                } else {
                    println!("config.toml is not setup correctly. Create new with 'virex config'.")
                }
            }
            "uninstall" => {
                if config::exists() && config::has_source() {
                    // uninstall function here...
                } else {
                    println!("config.toml is not setup correctly. Create new with 'virex config'.")
                }
            }
            "help" => {
                help::display();
            }
            "config" => {
                if !config::exists() {
                    let choice = &get_input("Config doesn't exist. Create new config? [y/n] ");
                    if choice == "y" || choice == "yes" || choice == "ye" {
                        match config::create() {
                            Ok(_) => {} // Config created without any error
                            Err(e) => println!("{}", e),
                        }
                    }
                } else if !config::has_source() {
                    let choice =
                        &get_input("Config doesn't have any source. Add new source? [y/n] ");
                    if choice == "y" || choice == "yes" || choice == "ye" {
                        let name = &get_input("Enter source name: ");
                        let url = &get_input("Enter source url: ");
                        match config::add_source(name, url) {
                            Ok(_) => {} // Source added without any error
                            Err(e) => println!("{}", e),
                        }
                    }
                } else {
                    let choice = &get_input("Add or remove source? [a/r] ");
                    if choice == "a" || choice == "ad" || choice == "add" {
                        let name = &get_input("Enter source name: ");
                        let url = &get_input("Enter source url: ");
                        match config::add_source(name, url) {
                            Ok(_) => {} // Source added without any error
                            Err(e) => println!("{}", e),
                        }
                    } else if choice == "r"
                        || choice == "re"
                        || choice == "rem"
                        || choice == "remo"
                        || choice == "remov"
                        || choice == "remove"
                    {
                        let name = &get_input("Enter source name: ");
                        match config::remove_source(name) {
                            Ok(_) => {} // Source removed without any error
                            Err(e) => println!("{}", e),
                        }
                    }
                }
            }
            "version" => {
                let name: &str = info::get_current_name();
                let version: &str = info::get_current_version();
                let authors: &str = info::get_current_authors();
                let description: &str = info::get_current_description();
                println!("{} - {}\nv{} by {}.", name, description, version, authors);
            }
            unknown => {
                println!("Unknown command: {}.", unknown);
            }
        }
    }
}

fn get_input(prompt: &str) -> String {
    print!("{}", prompt);
    stdout().flush().expect("Failed to flush");
    let mut input: String = String::new();
    stdin().read_line(&mut input).expect("Failed to readline");
    input.trim().to_lowercase()
}

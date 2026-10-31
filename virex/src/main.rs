use std::{
    env::args,
    io::{Write, stdin, stdout},
};

mod colors;
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
                if !errors_exists() {
                    // install function here...
                }
            }
            "update" => {
                if !errors_exists() {
                    // update function here...
                }
            }
            "uninstall" => {
                if !errors_exists() {
                    // uninstall function here...
                }
            }
            "help" => {
                help::display();
            }
            "config" => {
                if !config::exists() {
                    let choice = &get_input("Config doesn't exist. Create new config? [y/n] ");
                    if choice.starts_with('y') {
                        match config::create() {
                            Ok(_) => {} // Config created without any error
                            Err(e) => println!("{}", e),
                        }
                    }
                } else if !config::has_source() {
                    let choice =
                        &get_input("Config doesn't have any source. Add new source? [y/n] ");
                    if choice.starts_with('y') {
                        let name = &get_input("Enter source name: ");
                        let url = &get_input("Enter source url: ");
                        match config::add_source(name, url) {
                            Ok(_) => {} // Source added without any error
                            Err(e) => println!("{}", e),
                        }
                    }
                } else {
                    let choice = &get_input("Add or remove source? [a/r] ");
                    if choice.starts_with('a') {
                        let name = &get_input("Enter source name: ");
                        let url = &get_input("Enter source url: ");
                        match config::add_source(name, url) {
                            Ok(_) => {} // Source added without any error
                            Err(e) => println!("{}", e),
                        }
                    } else if choice.starts_with('r') {
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
                colors::yellow("Unknown command", unknown);
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

fn errors_exists() -> bool {
    if !config::exists() || !config::has_source() {
        colors::red(
            "config.toml is not setup correctly.",
            "Create new with 'virex config'.",
        );
        true
    } else {
        false
    }
}

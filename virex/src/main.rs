use std::env::args;

mod config;
mod help;
mod info;
mod install;
mod uninstall;
mod update;

fn main() {
    let args: Vec<String> = args().collect();

    // Skips name of package manager and checks all arguments
    let mut iter = args.iter().skip(1).peekable();

    while let Some(arg) = iter.next() {
        match arg.as_str() {
            "install" => {
                // install function here...
            }
            "update" => {
                // update function here...
            }
            "uninstall" => {
                // uninstall function here...
            }
            "help" => {
                help::display();
            }
            "version" => {
                let name = info::get_current_name();
                let version = info::get_current_version();
                let authors = info::get_current_authors();
                let description = info::get_current_description();
                println!("{} - {}\nv{} by {}.", name, description, version, authors);
            }
            other => {
                println!("Unknown command: {}", other);
            }
        }
    }
}

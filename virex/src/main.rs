use std::env;

mod config;
mod help;
mod info;
mod install;
mod uninstall;
mod update;

fn main() {
    let args: Vec<String> = env::args().collect();
    for arg in args {
        match arg.as_str() {
            "version" => {
                let name = info::get_current_name();
                let version = info::get_current_version();
                let authors = info::get_current_authors();
                let description = info::get_current_description();
                println!("{} - {}\nv{} by {}.", name, description, version, authors);
            }
            "help" => help::display(),
            _ => {}
        }
    }
}

const HELP: &'static str = "virex help - shows help information
virex install [package_name] - installs package
virex uninstall [package_name] - uninstalls package
virex config - allows you to edit configuration of virex
virex version - shows info about virex.";

pub fn display() {
    println!("{}", HELP);
}

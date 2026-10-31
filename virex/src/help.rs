const HELP: &'static str = "virex help - shows help information
virex install ... - installs package
virex uninstall ... - uninstalls package
virex config - allows you to edit configuration of virex
virex version - shows info about virex.";

pub fn display() {
    println!("{}", HELP);
}

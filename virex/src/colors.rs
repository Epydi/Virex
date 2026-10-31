// In developpement
// rivnarc

pub fn red(collored_prompt: &str, normal_prompt: &str) {
    println!("\x1b[1;31m{}\x1b[0m {}", collored_prompt, normal_prompt);
}

pub fn yellow(collored_prompt: &str, normal_prompt: &str) {
    println!("\x1b[1;33m{}\x1b[0m {}", collored_prompt, normal_prompt);
}

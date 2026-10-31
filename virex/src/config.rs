// In developpement
// rivnarc
use serde::{Deserialize, Serialize};
use std::{
    collections::HashMap,
    error::Error,
    fs::{read_to_string, write},
    path::Path,
};

#[derive(Serialize, Deserialize, Debug)]
pub struct Config {
    sources: HashMap<String, String>,
}

pub fn exists() -> bool {
    Path::new("src/config.toml").exists()
}

pub fn create() -> Result<(), Box<dyn Error>> {
    let initial_config = Config {
        sources: HashMap::from([("github".to_string(), "https://github.com/".to_string())]),
    };
    let toml_string = toml::to_string(&initial_config)?;
    write("src/config.toml", toml_string)?;
    Ok(())
}

pub fn edit(name: &str, url: &str) -> Result<(), Box<dyn Error>> {
    let path = "src/config.toml";
    let contents = read_to_string(path)?;
    let mut config: Config = toml::from_str(&contents)?;

    config.sources.insert(name.to_string(), url.to_string());
    let updated_toml = toml::to_string(&config)?;
    write(path, updated_toml)?;

    Ok(())
}

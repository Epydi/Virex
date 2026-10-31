// In developpement
// rivnarc
use serde::{Deserialize, Serialize};
use std::{
    collections::HashMap,
    error::Error,
    fs::{read_to_string, write},
    path::Path,
};

const PATH: &str = "src/config.toml";

#[derive(Serialize, Deserialize, Debug)]
pub struct Config {
    sources: HashMap<String, String>,
}

pub fn exists() -> bool {
    Path::new(PATH).exists()
}

pub fn create() -> Result<(), Box<dyn Error>> {
    let initial_config = Config {
        sources: HashMap::from([("github".to_string(), "https://github.com/".to_string())]),
    };
    let toml_string = toml::to_string(&initial_config)?;
    write(PATH, toml_string)?;
    Ok(())
}

pub fn add_source(name: &str, url: &str) -> Result<(), Box<dyn Error>> {
    let contents = read_to_string(PATH)?;
    let mut config: Config = toml::from_str(&contents)?;

    config.sources.insert(name.to_string(), url.to_string());
    let updated_toml = toml::to_string(&config)?;
    write(PATH, updated_toml)?;

    Ok(())
}

pub fn remove_source(name: &str) -> Result<(), Box<dyn Error>> {
    let contents = read_to_string(PATH)?;
    let mut config: Config = toml::from_str(&contents)?;
    let removed = config.sources.remove(name).is_some();
    if removed {
        let updated_toml = toml::to_string(&config)?;
        write(PATH, updated_toml)?;
    }
    Ok(())
}

pub fn has_source() -> bool {
    let contents = match std::fs::read_to_string(PATH) {
        Ok(c) => c,
        Err(_) => return false,
    };
    let config: Config = match toml::from_str(&contents) {
        Ok(cfg) => cfg,
        Err(_) => return false,
    };
    !config.sources.is_empty()
}

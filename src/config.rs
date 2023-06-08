use std::fs;
use directories::ProjectDirs;
use serde::Deserialize;


#[derive(Deserialize, Debug)]
pub struct Config {

}

impl Config {
    pub fn build() -> Result<Config, String> {
        if let Some(proj_dirs) = ProjectDirs::from(
            "dev",
            "vympel",
            "vympel",
        ) {
            let config_dir = proj_dirs.config_dir();
            
            let config_file = fs::read_to_string(
                config_dir.join("config.toml")
            );

            let config: Config = match config_file {
                Ok(file) => {
                    match toml::from_str(&file) {
                        Ok(toml_deserialize) => toml_deserialize,
                        Err(err) => return Err(format!("Error in config: {}", err.message().to_string())),
                    }
                },
                Err(_) => Config {
                }
            };
            
            return Ok(config);
        }
        Err(String::from("No valid home directory path"))
    }
}

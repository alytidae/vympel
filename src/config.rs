use std::{fs, io};
use directories::ProjectDirs;
use serde::{Deserialize, Serialize};


#[derive(Deserialize, Serialize, Debug)]
pub struct Config {
    pub tasks_folder_path: String,
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
                Err(_) => {
                    match fs::create_dir(config_dir.join("tasks")){
                        Err(err) => {
                            if err.kind() != io::ErrorKind::AlreadyExists {
                                return Err(format!("Error in creation tasks folder: {}", err.to_string()));
                            }
                        }, 
                        _ => (),
                    }
                    let config = Config {
                        tasks_folder_path: config_dir.join("tasks").display().to_string(),
                    };
                    fs::write(config_dir.join("config.toml"), toml::to_string(&config).unwrap()).unwrap(); 
                    config
                } 
            };
            
            return Ok(config);
        }
        Err(String::from("No valid home directory path"))
    }
}

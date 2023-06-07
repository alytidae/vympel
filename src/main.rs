use std::fs;
use std::process;
use directories::ProjectDirs;
use serde::Deserialize;
use clap::{Parser, Subcommand, ValueEnum};

#[derive(Debug, Parser)]
#[clap(author, version, about)]
struct AppArgs {
    #[clap(subcommand)]
    command: Option<Command>,
}

#[derive(Debug, Subcommand)]
enum Command {
    #[clap(skip)]
    Show,
    /// Add task 
    Add{
        task_body: String, 
        task_priority: Option<TaskPriority>,
    },
    /// Remove task
    Rm {
        /// Number of task to remove
        task_number: u32,
    },
    /// Show completed and incomplete tasks
    All,
    // TODO: Edit(u32),
}

#[derive(Debug, ValueEnum, Clone)]
enum TaskPriority {
    Low,
    /// Default value
    Mid,
    High,
}

#[derive(Deserialize, Debug)]
struct Config {
}

impl Command {
    fn show(&self, config: &Config) -> Result<(), &'static str>{
        todo!();
    }
    
    fn add(&self, config: &Config) -> Result<(), &'static str>{
        todo!();
    }
    
    fn rm(&self, config: &Config) -> Result<(), &'static str>{
        todo!();
    }
    
    fn all(&self, config: &Config) -> Result<(), &'static str>{
        todo!();
    }

    fn execute(&self, config: &Config) -> Result<(), &'static str>{
        match self {
            Command::Show => self.show(config),
            Command::Add { .. } => self.add(config),
            Command::Rm { .. } => self.rm(config),
            Command::All => self.all(config),
        }
    }
}

impl Config {
    fn build() -> Result<Config, String> {
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

fn main() {    
    let config = Config::build().unwrap_or_else(|err| {
        println!("{err}");
        process::exit(1);
    });
    
    let args = AppArgs::parse();
    let command = args.command.unwrap_or(Command::Show);
    dbg!(command);

    //match command.execute(&config) {
    //    Ok(_) => (),
    //    Err(error) => println!("{error}"),
    //};

}

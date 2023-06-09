use crate::tasks::Tasks;
use crate::Command;
use crate::Config;

use std::fs;

impl Command {
    fn show(&self, config: &Config) -> Result<(), &'static str> {
        println!("{}", config.tasks_folder_path);
        let entries = fs::read_dir(&config.tasks_folder_path);
        let tasks = Tasks::build(entries.unwrap()).unwrap();

        tasks.print_short_list();
        //if let Ok(entries) = fs::read_dir(&config.tasks_folder_path) {
        //    for entry in entries {
        //        if let Ok(entry) = entry {
        //            // Here, `entry` is a `DirEntry`.
        //            println!("{}", entry.path().file_stem().unwrap().to_str().unwrap());
        //        }
        //    }
        //}
        Ok(())
    }

    fn add(&self, config: &Config) -> Result<(), &'static str> {
        todo!();
    }

    fn rm(&self, config: &Config) -> Result<(), &'static str> {
        todo!();
    }

    fn all(&self, config: &Config) -> Result<(), &'static str> {
        todo!();
    }

    pub fn execute(&self, config: &Config) -> Result<(), &'static str> {
        match self {
            Command::Show => self.show(config),
            Command::Add { .. } => self.add(config),
            Command::Rm { .. } => self.rm(config),
            Command::All => self.all(config),
        }
    }
}

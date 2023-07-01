use std::fs::{self, ReadDir};
use std::path::{PathBuf};

pub struct Tasks {
    list: Vec<Task>,
}

pub struct Task {
    name: String,
    status: TaskStatus,
    creation_date: String,
    complete_date: Option<String>,
    description: String,
}

pub enum TaskStatus {
    Active,
    Inactive,    
}

impl Tasks {
    //TODO: Return tasks
    pub fn build(entries: ReadDir) -> Result<(), String> {
        for entry in entries {
            Task::build(entry.unwrap().path());
        }
        Ok(())
    }

}

impl Task {
    //TODO: Maybe i need to change this md parser...
    fn build(path: PathBuf) -> Result<(), String> {
        let data = fs::read_to_string(path).unwrap();
        let parts = data.split("---\n").collect::<Vec<&str>>();
        // We have something before metadata, then there is metadata after them there is a description
        if parts.len() == 3 {
            for meta_field in parts[1].lines(){
                println!("{}", meta_field);
            } 
        }
        Ok(()) 
    }
}

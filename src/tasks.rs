use std::fs::{self, ReadDir};

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
            println!("{:?}", entry);
        }
        Ok(())
    }
}

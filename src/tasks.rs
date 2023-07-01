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
            println!("###");
            let data = fs::read_to_string(entry.unwrap().path()).unwrap();
            let parts = data.split("---").collect::<Vec<&str>>();
            if parts.len() > 2 {
                println!("{:?}", parts[1]);
            }
            println!("###");
        }
        Ok(())
    }

}

impl Task {
    //TODO: Maybe i need to change this md parser...
    fn build(path: String) -> Result<(), String> {
        Ok(()) 
    }
}

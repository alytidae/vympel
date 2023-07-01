use std::collections::HashMap;
use std::fs::{self, ReadDir};
use std::io::{self, Write};
use std::path::PathBuf;

pub struct Tasks {
    list: Vec<Task>,
}

pub struct Task {
    name: String,
    status: TaskStatus,
    creation_date: String,
    complete_date: Option<String>,
    description: Option<String>,
}

pub enum TaskStatus {
    Active,
    Inactive,
}

impl Tasks {
    //TODO: Return tasks
    pub fn build(entries: ReadDir) -> Result<Tasks, String> {
        let mut task_list = Vec::new();
        for entry in entries {
            task_list.push(Task::build(entry.unwrap().path()).unwrap());
        }

        Ok(Tasks { list: task_list })
    }

    pub fn print_short_list(&self) {
        for i in 0..self.list.len() {
            print!("{}. ", i + 1);
            io::stdout().flush().unwrap();
            self.list[i].print_short();
        }
    }
}

impl Task {
    //TODO: Maybe i need to change this md parser...
    fn build(path: PathBuf) -> Result<Task, String> {
        let data = fs::read_to_string(path).unwrap();
        let parts = data.split("---\n").collect::<Vec<&str>>();

        let mut metadata: HashMap<String, String> = HashMap::new();

        // We have something before metadata, then there is metadata after them there is a description
        if parts.len() == 3 {
            for meta_field in parts[1].lines() {
                let pair = meta_field.splitn(2, ":").collect::<Vec<&str>>();
                let meta_key = pair[0];
                let meta_value = pair[1].replace("\"", "");
                metadata.insert(meta_key.to_string(), meta_value);
            }
            
            //TODO: rewrite, its awful
            let task_name = metadata.get("name").unwrap().trim().to_string();
            let task_status;
            if metadata.get("status").unwrap().trim().to_string() == "active" {
                task_status = TaskStatus::Active;
            }else if metadata.get("status").unwrap().trim().to_string() == "inactive" {
                task_status = TaskStatus::Inactive;
            }else {
                return Err(String::from("Cant parse status field in task"));
            }
            let task_creation_date = metadata.get("creation_date").unwrap().trim().to_string();
            let task_complete_date;
            if metadata.get("complete_date").unwrap().trim().to_string() == "" {
                task_complete_date = None;
            }else {
                task_complete_date = Some(metadata.get("complete_date").unwrap().trim().to_string());
            }
            let task_description;
            if parts[2] == "" {
                task_description = None;
            }else {
                task_description = Some(parts[2].trim().to_string());
            }

            let task = Task {
                name: task_name,
                status: task_status,
                creation_date: task_creation_date,
                complete_date: task_complete_date,
                description: task_description,
            };
            
            return Ok(task)
        }

        Err(String::from("Failed to parse task"))
    }

    pub fn print_short(&self) {
        println!("{}", &self.name);
    }

    pub fn print_full(&self) {
        todo!();
    }
}

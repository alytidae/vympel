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
    pub fn build(entries: ReadDir) -> Result<Tasks, String> {
        let mut task_list = Vec::new();
        for entry in entries {
            task_list.push(Task::build(entry.unwrap().path()).unwrap());
        }

        Ok(Tasks{ list: task_list })
    }

}

impl Task {
    //TODO: Maybe i need to change this md parser...
    fn build(path: PathBuf) -> Result<Task, String> {
        let data = fs::read_to_string(path).unwrap();
        let parts = data.split("---\n").collect::<Vec<&str>>();
        let task = Task{
            name: String::from("123"),
            status: TaskStatus::Active,
            creation_date: String::from("123"),
            complete_date: Some(String::from("123")),
            description: String::from("123"),
        };
        // We have something before metadata, then there is metadata after them there is a description
        //if parts.len() == 3 {
        //    for meta_field in parts[1].lines(){
        //        println!("{}", meta_field);
        //    } 
        //}

        Ok(task)
    }

    pub fn short_print(&self) {
        println!("{}", &self.name);
    }

    pub fn full_print(&self) {
        todo!();
    }
}

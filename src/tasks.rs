struct Tasks {
    list: Vec<Task>,
}

struct Task {
    name: String,
    status: TaskStatus,
    creation_date: String,
    complete_date: Option<String>,
}

enum TaskStatus {
    Active,
    Inactive,    
}
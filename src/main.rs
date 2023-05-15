enum Command {
    Show,
    Add,
    Rm,
    All,
}

enum Arguments {
    Message(String, String),
    Number(u32),
}
struct Config {
    command: Command,
    arguments: Arguments,
}

fn show_help() {
    println!("There are several commands that you can use:\n");
    println!("tasks add \"Buy a milk\"         - add task \"Buy a milk\"");
    println!("tasks add \"Watch a film\" high  - add a priority task (low, mid:default, high)");
    println!("tasks rm 2                     - delete task with number 2");
    println!("tasks all                      - show completed and incomplete tasks");
}

fn main() {    
}

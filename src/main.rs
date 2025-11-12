mod task;
mod todolist;
use task::Task;
use todolist::ToDoList;

use std::io;
fn main() {

    println!("Please enter your name:");

    let mut name = String::new();

    io::stdin()
    .read_line(&mut name)
    .unwrap();

    let name = name.trim().to_string();

    let mut todo_list = ToDoList::new(name);
    println!("Successfully created a to-do list for {}!", todo_list.owner);

    println!("Number of tasks: {}", todo_list.tasks.len());
    print!("Would you like to add a task? (y/n): ");

    
    let mut add_task = String::new();
    io::stdin()
        .read_line(&mut add_task)
        .unwrap();

    if add_task.trim().eq_ignore_ascii_case("y") {
        println!("Please enter the task name:");

        let mut task_name = String::new();
        io::stdin()
            .read_line(&mut task_name)
            .unwrap();

        let task_name = task_name.trim().to_string();
        let task = Task::new(todo_list.tasks.len() as u32 + 1, task_name);
        todo_list.add_task(task);
        println!("Task added successfully!");
    }

    println!("Here are your tasks:");
    todo_list.list_tasks();
}

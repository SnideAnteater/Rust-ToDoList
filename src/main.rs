mod task;
mod todolist;
use task::Task;
use todolist::ToDoList;

use std::io::{self, Write};

fn main() {
    println!("Please enter your name:");
    let mut name = String::new();
    io::stdin().read_line(&mut name).unwrap();
    let name = name.trim().to_string();

    let mut todo_list = ToDoList::new(name);
    println!("Successfully created a to-do list for {}!", todo_list.owner);

    loop {
        println!("\n=== To-Do List Menu ===");
        println!("1. Add task");
        println!("2. List tasks");
        println!("3. Mark task as completed");
        println!("4. Remove task");
        println!("5. Exit");
        print!("Choose an option: ");
        io::stdout().flush().unwrap();

        let mut choice = String::new();
        io::stdin().read_line(&mut choice).unwrap();

        match choice.trim() {
            "1" => add_task(&mut todo_list),
            "2" => todo_list.list_tasks(),
            "3" => mark_completed(&mut todo_list),
            "4" => remove_task(&mut todo_list),
            "5" => {
                println!("Goodbye!");
                break;
            }
            _ => println!("Invalid option, please try again."),
        }
    }
}

fn add_task(todo_list: &mut ToDoList) {
    print!("Enter task name: ");
    io::stdout().flush().unwrap();
    
    let mut task_name = String::new();
    io::stdin().read_line(&mut task_name).unwrap();
    let task_name = task_name.trim().to_string();
    
    if task_name.is_empty() {
        println!("Task name cannot be empty!");
        return;
    }
    
    let task = Task::new(todo_list.tasks.len() as u32 + 1, task_name);
    todo_list.add_task(task);
    println!("Task added successfully!");
}

fn mark_completed(todo_list: &mut ToDoList) {
    if todo_list.tasks.is_empty() {
        println!("No tasks to mark as completed!");
        return;
    }
    
    todo_list.list_tasks();
    print!("Enter task ID to mark as completed: ");
    io::stdout().flush().unwrap();
    
    let mut id = String::new();
    io::stdin().read_line(&mut id).unwrap();
    
    if let Ok(id) = id.trim().parse::<u32>() {
        todo_list.mark_task_completed(id);
    } else {
        println!("Invalid ID!");
    }
}

fn remove_task(todo_list: &mut ToDoList) {
    if todo_list.tasks.is_empty() {
        println!("No tasks to remove!");
        return;
    }
    
    todo_list.list_tasks();
    print!("Enter task ID to remove: ");
    io::stdout().flush().unwrap();
    
    let mut id = String::new();
    io::stdin().read_line(&mut id).unwrap();
    
    if let Ok(id) = id.trim().parse::<u32>() {
        todo_list.remove_task(id);
    } else {
        println!("Invalid ID!");
    }
}

use crate::task::Task;

pub struct ToDoList {
    pub owner: String,
    pub tasks: Vec<Task>,
}

impl ToDoList {
    pub fn new(owner: String) -> Self {
        ToDoList {
            owner,
            tasks: Vec::new(),
        }
    }

    pub fn add_task(&mut self, task: Task) {
        self.tasks.push(task);
    }

    pub fn list_tasks(&self) {
        if self.tasks.is_empty() {
            println!("No tasks found!");
            return;
        }
        
        println!("\n=== Your Tasks ===");
        for task in &self.tasks {
            let status = if task.completed { "✓" } else { "✗" };
            println!(
                "[{}] ID: {}, Name: {}",
                status, task.id, task.name
            );
        }
    }

    pub fn mark_task_completed(&mut self, id: u32) {
        if let Some(task) = self.tasks.iter_mut().find(|t| t.id == id) {
            task.mark_completed();
            println!("Task '{}' marked as completed!", task.name);
        } else {
            println!("Task with ID {} not found!", id);
        }
    }

    pub fn remove_task(&mut self, id: u32) {
        if let Some(pos) = self.tasks.iter().position(|t| t.id == id) {
            let task = self.tasks.remove(pos);
            println!("Task '{}' removed!", task.name);
        } else {
            println!("Task with ID {} not found!", id);
        }
    }
}
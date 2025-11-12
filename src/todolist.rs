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
        for task in &self.tasks {
            println!(
                "ID: {}, Name: {}, Completed: {}",
                task.id, task.name, task.completed
            );
        }
    }
}
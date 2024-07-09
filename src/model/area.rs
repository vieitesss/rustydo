use uuid::Uuid;

use super::task::Task;

#[derive(Clone)]
pub struct Area {
    id: Uuid,
    title: String,
    tasks: Vec<Task>,
}

impl Area {
    pub fn new(title: &str) -> Self {
        Area {
            id: Uuid::new_v4(),
            title: title.into(),
            tasks: Vec::new(),
        }
    }

    pub fn get_id(&self) -> Uuid {
        self.id
    }

    pub fn get_title(&self) -> String {
        self.title.clone()
    }

    pub fn set_title(&mut self, title: String) {
        self.title = title;
    }

    //TODO: update, delete tasks

    pub fn push_task(&mut self, task: Task) {
        self.tasks.push(task);
    }

    pub fn get_tasks(&mut self) -> &mut [Task] {
        &mut self.tasks
    }
}

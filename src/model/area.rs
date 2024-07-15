use uuid::Uuid;

use super::task::Task;

#[derive(Clone)]
pub struct Area {
    id: Uuid,
    pub title: String,
    pub tasks: Vec<Task>,
}

impl Area {
    pub fn new(title: &str) -> Self {
        Area {
            id: Uuid::new_v4(),
            title: title.into(),
            tasks: Vec::new(),
        }
    }

    pub fn id(&self) -> Uuid {
        self.id
    }

    //TODO: update, delete tasks
}

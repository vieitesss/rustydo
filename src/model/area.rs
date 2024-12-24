use super::task::Task;
use uuid::Uuid;

#[derive(Debug, Clone, PartialEq)]
pub struct Area {
    id: Uuid,
    pub title: String,
    pub tasks: Vec<Task>,
    pub selected: bool,
}

impl Area {
    pub fn new(title: &str) -> Self {
        Area {
            id: Uuid::new_v4(),
            title: title.into(),
            tasks: Vec::new(),
            selected: false,
        }
    }

    pub fn id(&self) -> &Uuid {
        &self.id
    }
}

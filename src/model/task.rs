use uuid::Uuid;

#[derive(Clone)]
pub struct Task {
    id: Uuid,
    area: Uuid,
    pub title: String,
    pub done: bool,
}

impl Task {
    pub fn new(title: &str, area: Uuid) -> Self {
        Task {
            id: Uuid::new_v4(),
            title: title.to_string(),
            done: false,
            area,
        }
    }

    pub fn id(&self) -> &Uuid {
        &self.id
    }

    pub fn area(&self) -> &Uuid {
        &self.area
    }
}

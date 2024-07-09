#![allow(dead_code)]

use uuid::Uuid;

#[derive(Clone)]
pub struct Task {
    id: Uuid,
    area: Uuid,
    title: String,
    done: bool,
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

    pub fn is_done(&self) -> bool {
        self.done
    }

    pub fn set_done(&mut self, done: bool) {
        self.done = done;
    }

    pub fn set_title(&mut self, title: String) {
        self.title = title;
    }

    pub fn get_title(&self) -> String {
        self.title.clone()
    }

    pub fn get_area(&self) -> Uuid {
        self.area.clone()
    }
}

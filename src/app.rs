#![allow(dead_code)]

#[derive(PartialEq)]
pub enum AppStatus {
    // TODO: more status?
    Running,
    Quitting,
}

pub struct App {
    // TODO: improve
    pub status: AppStatus,
}

impl Default for App {
    fn default() -> Self {
        App {
            status: AppStatus::Running,
        }
    }
}

impl App {
    pub fn new() -> Self {
        Self::default()
    }

    pub fn is_running(&self) -> bool {
        self.status == AppStatus::Running
    }
}

#![allow(dead_code)]

use crate::model::task::Task;

#[derive(PartialEq, Clone)]
pub enum AppStatus {
    // TODO: more states?
    Running,
    Quitting,
}

#[derive(PartialEq, Clone)]
pub enum AppWindow {
    Main,
}

pub struct App {
    // TODO: improve
    status: AppStatus,
    window: AppWindow,
    tasks: Vec<Task>,
}

impl Default for App {
    fn default() -> Self {
        let mut tasks: Vec<Task> = vec![
            Task::new("hacer la maleta"),
            Task::new("recoger el ordenador"),
            Task::new("lavarme los dientes"),
            Task::new("mirar la nómina"),
        ];
        tasks[1].set_done(true);

        App {
            status: AppStatus::Running,
            window: AppWindow::Main,
            tasks,
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

    pub fn get_window(&self) -> AppWindow {
        self.window.clone()
    }

    pub fn set_window(&mut self, window: AppWindow) {
        self.window = window;
    }

    pub fn get_status(&self) -> AppStatus {
        self.status.clone()
    }

    pub fn set_status(&mut self, status: AppStatus) {
        self.status = status;
    }

    pub fn get_tasks(&self) -> &Vec<Task> {
        &self.tasks
    }
}

#![allow(dead_code)]

use crate::model::{area::Area, task::Task};

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
    areas: Vec<Area>,
    current_area: Option<String>,
}

impl Default for App {
    fn default() -> Self {
        let mut area = Area::new("Universidad");
        area.push_task(Task::new("hacer la maleta", area.get_id()));
        area.push_task(Task::new("recoger el ordenador", area.get_id()));
        area.push_task(Task::new("lavarme los dientes", area.get_id()));
        area.push_task(Task::new("mirar la nómina", area.get_id()));
        area.get_tasks()[1].set_done(true);

        App {
            status: AppStatus::Running,
            window: AppWindow::Main,
            areas: vec![area],
            current_area: Some("Universidad".to_string()),
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

    pub fn get_current_area(&self) -> Option<Area> {
        match &self.current_area {
            Some(area) => self.get_area_by_title(&area),
            None => None,
        }
    }

    fn get_area_by_title(&self, title: &str) -> Option<Area> {
        for area in &self.areas {
            if area.get_title() == title {
                return Some(area.clone());
            }
        }

        None
    }
}

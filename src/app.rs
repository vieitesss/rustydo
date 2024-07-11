#![allow(unreachable_patterns)]

use crate::{
    model::{area::Area, task::Task},
    util::input::Input,
};

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

#[derive(PartialEq, Clone)]
pub enum WindowPane {
    Areas,
    Tasks,
    Input,
}

pub struct App {
    status: AppStatus,
    window: AppWindow,
    pane: WindowPane,
    prev_pane: Option<WindowPane>,
    areas: Vec<Area>,
    current_area: Option<String>,
    input: Input,
    // cursor_offset: u16,
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
            pane: WindowPane::Tasks,
            prev_pane: None,
            areas: vec![area],
            current_area: Some("Universidad".to_string()),
            input: Input::new(),
            // cursor_offset: 0,
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

    pub fn get_pane(&self) -> WindowPane {
        self.pane.clone()
    }

    pub fn set_pane(&mut self, pane: WindowPane) {
        self.pane = pane;
    }

    pub fn get_prev_pane(&self) -> Option<WindowPane> {
        self.prev_pane.clone()
    }

    pub fn save_current_pane(&mut self) {
        self.prev_pane = Some(self.get_pane());
    }

    pub fn get_status(&self) -> AppStatus {
        self.status.clone()
    }

    pub fn set_status(&mut self, status: AppStatus) {
        self.status = status;
    }

    pub fn get_input(&mut self) -> &mut Input {
        &mut self.input
    }

    pub fn handle_input_text(&mut self) {
        match &self.prev_pane {
            Some(pane) => match pane {
                WindowPane::Areas => self.new_area(),
                WindowPane::Tasks => todo!("New task"),
                WindowPane::Input => panic!("Input pane should not be a previous pane"),
            },
            None => panic!("Should be a previous pane"),
        }
    }

    pub fn clear_input_text(&mut self) {
        self.input.clear();
    }

    pub fn get_areas(&self) -> &[Area] {
        &self.areas
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

    pub fn focus_next(&mut self) {
        match self.get_pane() {
            WindowPane::Areas => self.set_pane(WindowPane::Tasks),
            WindowPane::Tasks => self.set_pane(WindowPane::Areas),
            _ => {}
        }
    }

    pub fn insert_char(&mut self, c: char) {
        self.input.insert_char(c);
    }

    pub fn remove_char(&mut self) {
        self.input.remove_char();
    }

    pub fn new_area(&mut self) {
        let area = Area::new(self.input.get_text().trim());
        self.areas.push(area);

        self.input.clear();
    }
}

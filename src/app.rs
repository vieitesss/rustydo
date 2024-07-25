#![allow(unreachable_patterns)]

use crate::{model::{area::Area, task::Task}, util::input::Input};

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
    pub status: AppStatus,
    pub window: AppWindow,
    pub pane: WindowPane,
    pub prev_pane: Option<WindowPane>,
    pub areas: Vec<Area>,
    /// The area whose tasks are shown
    pub current_area: usize,
    /// The area selected in the list
    pub selected_area: usize,
    pub input: Input,
    // cursor_offset: u16,
}

impl Default for App {
    fn default() -> Self {
        let mut uni = Area::new("Universidad");
        uni.tasks.push(Task::new("hacer la maleta", *uni.id()));
        let mut casa = Area::new("Casa");
        casa.tasks.push(Task::new("recoger la habitación", *casa.id()));
        // area.tasks
        //     .push(Task::new("recoger el ordenador", *area.id()));
        // area.tasks
        //     .push(Task::new("lavarme los dientes", *area.id()));
        // area.tasks.push(Task::new("mirar la nómina", *area.id()));
        // area.tasks[1].done = true;

        App {
            status: AppStatus::Running,
            window: AppWindow::Main,
            pane: WindowPane::Tasks,
            prev_pane: None,
            areas: vec![uni, casa],
            current_area: 0,
            selected_area: 0,
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

    pub fn save_current_pane(&mut self) {
        self.prev_pane = Some(self.pane.clone());
    }

    pub fn set_prev_pane(&mut self) {
        match &self.prev_pane {
            Some(pane) => self.pane = pane.clone(),
            None => panic!("There should be a pane!"),
        }
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

    pub fn focus_next(&mut self) {
        match self.pane {
            WindowPane::Areas => self.pane = WindowPane::Tasks,
            WindowPane::Tasks => self.pane = WindowPane::Areas,
            _ => {}
        }
    }

    pub fn new_area(&mut self) {
        let area = Area::new(self.input.text.trim());
        self.areas.push(area);

        self.input.clear();
    }

    pub fn next_area(&mut self) {
        self.selected_area = (self.selected_area as usize + 1) % self.areas.len()
    }

    pub fn prev_area(&mut self) {
        if self.selected_area as usize == 0 {
            self.selected_area = self.areas.len() - 1;
        } else {
            self.selected_area -= 1;
        }
    }

    pub fn update_current_area(&mut self) {
        self.current_area = self.selected_area
    }
}

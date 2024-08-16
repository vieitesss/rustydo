use crate::components::{areas::Areas, input::Input};

#[derive(PartialEq, Clone)]
pub enum AppStatus {
    Running,
    Quitting,
}

#[derive(PartialEq, Clone)]
pub enum AppWindow {
    Main,
}

#[derive(PartialEq, Clone, Debug)]
pub enum Focus {
    Areas,
    Tasks,
    Input,
}

pub struct App {
    pub status: AppStatus,
    pub window: AppWindow,
    pub focus: Focus,
    pub prev_focus: Option<Focus>,
    pub areas: Areas,
    pub input: Input,
}

impl Default for App {
    fn default() -> Self {
        App {
            status: AppStatus::Running,
            window: AppWindow::Main,
            focus: Focus::Tasks,
            prev_focus: None,
            input: Input::default(),
            areas: Areas::default(),
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

    pub fn focus_input(&mut self) {
        self.save_current_pane();
        self.focus = Focus::Input;
    }

    pub fn save_current_pane(&mut self) {
        self.prev_focus = Some(self.focus.clone());
    }

    pub fn set_prev_pane(&mut self) {
        match &self.prev_focus {
            Some(pane) => self.focus = pane.clone(),
            None => panic!("There should be a pane!"),
        }
    }
}

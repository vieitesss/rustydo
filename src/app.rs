use crate::frames::{areas::Areas, input::Input};

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

}

use crate::handler::Action;
use ratatui::{crossterm::event::KeyCode, layout::Rect, Frame};

pub mod areas;
pub mod input;
pub mod tasks;

pub trait FrameTrait {
    fn render(&mut self, frame: &mut Frame, rect: Option<Rect>);
    fn handle_key(&mut self, key: KeyCode) -> Option<Action>;
    fn handle_action(&mut self, action: Action) -> Option<Action>;
}

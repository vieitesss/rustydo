use crate::handler::Action;
use ratatui::{layout::Rect, Frame};

pub mod areas;
pub mod input;
pub mod tasks;

pub trait Component {
    fn render(&mut self, frame: &mut Frame, rect: Option<Rect>);
    fn handle_action(&mut self, action: Action) -> Option<Action>;
}

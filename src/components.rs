use ratatui::{layout::Rect, Frame};

pub mod input;
pub mod areas;
pub mod tasks;

pub trait Component {
    fn render(&mut self, frame: &mut Frame, rect: Option<Rect>);
}

use ratatui::Frame;

pub mod input;

pub trait Component {
    fn render(&mut self, frame: &mut Frame);
}

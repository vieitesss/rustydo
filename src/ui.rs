use crate::app::App;

use ratatui::{style::Stylize, widgets::Paragraph, Frame};

pub fn render(frame: &mut Frame, _app: &mut App) {
    let area = frame.size();
    frame.render_widget(
        Paragraph::new("Hello Ratatui! (press 'q' to quit)")
            .red()
            .on_dark_gray(),
        area,
    );
}

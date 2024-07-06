// ----- TODO -----
//
// Create "App" struct with something like:
//     - tasks
//     - state
//     - ...
// 
// Improve project file structure:
//     - main.rs
//     - event.rs
//     - ui.rs

use std::io::{stdout, Result};

use ratatui::{
    backend::CrosstermBackend,
    crossterm::{
        event::{self, KeyCode, KeyEventKind},
        terminal::{disable_raw_mode, enable_raw_mode, EnterAlternateScreen, LeaveAlternateScreen},
        ExecutableCommand,
    },
    prelude::Backend,
    style::Stylize,
    widgets::Paragraph,
    Terminal,
};

fn draw_ui(terminal: &mut Terminal<impl Backend>) -> Result<()> {
    terminal.draw(|f| {
        let area = f.size();
        f.render_widget(
            Paragraph::new("Hello Ratatui! (press 'q' to quit)")
                .red()
                .on_dark_gray(),
            area,
        );
    })?;

    Ok(())
}

fn init_app() -> Result<Terminal<impl Backend>> {
    stdout().execute(EnterAlternateScreen)?;
    enable_raw_mode()?;
    let mut terminal = Terminal::new(CrosstermBackend::new(stdout()))?;
    terminal.clear()?;

    Ok(terminal)
}

fn quit_app() -> Result<()> {
    stdout().execute(LeaveAlternateScreen)?;
    disable_raw_mode()?;

    Ok(())
}

fn main() -> Result<()> {
    let mut terminal = init_app()?;

    loop {
        draw_ui(&mut terminal)?;

        // 16 millis = 60 fps
        if event::poll(std::time::Duration::from_millis(16))? {
            if let event::Event::Key(key) = event::read()? {
                if key.kind == KeyEventKind::Press {
                    match key.code {
                        KeyCode::Char('q') => break,
                        _ => {}
                    }
                }
            }
        }
    }

    quit_app()?;

    Ok(())
}

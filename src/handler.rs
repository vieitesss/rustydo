use crate::app::{App, AppStatus, AppWindow, WindowPane};
use std::io::Result;

use ratatui::crossterm::event::{self, KeyCode, KeyEventKind};

pub fn event(app: &mut App) -> Result<()> {
    if event::poll(std::time::Duration::from_millis(16))? {
        match event::read()? {
            event::Event::FocusGained => {}
            event::Event::FocusLost => {}
            event::Event::Key(key) => match key.kind {
                KeyEventKind::Press | KeyEventKind::Repeat => key_press(key.code, app),
                KeyEventKind::Release => {}
            },
            event::Event::Mouse(_) => {}
            event::Event::Paste(_) => {}
            event::Event::Resize(_, _) => {}
        }
    }

    Ok(())
}

fn key_press(key: KeyCode, app: &mut App) {
    match key {
        KeyCode::Backspace => {
            if app.get_pane() == WindowPane::Input {
                app.remove_char(false);
            }
        }
        KeyCode::Enter => {
            if app.get_pane() == WindowPane::Input {
                app.handle_input_text();
                app.set_pane(app.get_prev_pane().unwrap());
            }
        }
        KeyCode::Left => {}
        KeyCode::Right => {}
        KeyCode::Up => {}
        KeyCode::Down => {}
        KeyCode::Home => {}
        KeyCode::End => {}
        KeyCode::PageUp => {}
        KeyCode::PageDown => {}
        KeyCode::Tab => app.focus_next(),
        KeyCode::BackTab => {}
        KeyCode::Delete => {
            if app.get_pane() == WindowPane::Input {
                app.remove_char(true);
            }
        }
        KeyCode::Insert => {}
        KeyCode::F(_) => {}
        KeyCode::Char(char) => {
            if app.get_pane() != WindowPane::Input {
                match char {
                    'q' => app.set_status(AppStatus::Quitting),
                    'n' => {
                        app.save_current_pane();
                        app.set_pane(WindowPane::Input);
                    },
                    _ => {}
                }
            } else {
                app.insert_char(char);
            }
        }
        KeyCode::Null => {}
        KeyCode::Esc => {
            if app.get_pane() == WindowPane::Input {
                match app.get_window() {
                    AppWindow::Main => {
                        app.clear_input_text();
                        app.set_pane(app.get_prev_pane().unwrap());
                    },
                }
            }
        }
        KeyCode::CapsLock => {}
        KeyCode::ScrollLock => {}
        KeyCode::NumLock => {}
        KeyCode::PrintScreen => {}
        KeyCode::Pause => {}
        KeyCode::Menu => {}
        KeyCode::KeypadBegin => {}
        KeyCode::Media(_) => {}
        KeyCode::Modifier(_) => {}
    }
}

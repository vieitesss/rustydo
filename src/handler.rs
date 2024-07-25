use crate::app::{App, AppStatus, AppWindow, WindowPane};
use std::io::Result;

use ratatui::crossterm::event::{self, KeyCode, KeyEventKind};

pub fn event(app: &mut App) -> Result<()> {
    if event::poll(std::time::Duration::from_millis(16))? {
        match event::read()? {
            event::Event::FocusGained => (),
            event::Event::FocusLost => (),
            event::Event::Key(key) => match key.kind {
                KeyEventKind::Press | KeyEventKind::Repeat => key_press(key.code, app),
                KeyEventKind::Release => (),
            },
            event::Event::Mouse(_) => (),
            event::Event::Paste(_) => (),
            event::Event::Resize(_, _) => (),
        }
    }

    Ok(())
}

fn key_press(key: KeyCode, app: &mut App) {
    match key {
        KeyCode::Backspace => {
            if app.pane == WindowPane::Input {
                app.input.remove_char();
            }
        }
        KeyCode::Enter => match app.pane {
            WindowPane::Areas => app.update_current_area(),
            WindowPane::Tasks => (),
            WindowPane::Input => {
                app.handle_input_text();
                app.set_prev_pane();
            }
        },
        KeyCode::Left => (),
        KeyCode::Right => (),
        KeyCode::Up => (),
        KeyCode::Down => (),
        KeyCode::Home => (),
        KeyCode::End => (),
        KeyCode::PageUp => (),
        KeyCode::PageDown => (),
        KeyCode::Tab => app.focus_next(),
        KeyCode::BackTab => (),
        KeyCode::Delete => (),
        KeyCode::Insert => (),
        KeyCode::F(_) => (),
        KeyCode::Char(char) => {
            if app.pane != WindowPane::Input {
                match char {
                    'q' => app.status = AppStatus::Quitting,
                    'n' => {
                        app.save_current_pane();
                        app.pane = WindowPane::Input;
                    }
                    'j' => match app.pane {
                        WindowPane::Areas => app.next_area(),
                        WindowPane::Tasks => (),
                        WindowPane::Input => (),
                    }
                    'k' => match app.pane {
                        WindowPane::Areas => app.prev_area(),
                        WindowPane::Tasks => (),
                        WindowPane::Input => (),
                    }
                    _ => (),
                }
            } else {
                app.input.insert_char(char);
            }
        }
        KeyCode::Null => (),
        KeyCode::Esc => {
            if app.pane == WindowPane::Input {
                match app.window {
                    AppWindow::Main => {
                        app.input.clear();
                        app.set_prev_pane();
                    }
                }
            }
        }
        KeyCode::CapsLock => (),
        KeyCode::ScrollLock => (),
        KeyCode::NumLock => (),
        KeyCode::PrintScreen => (),
        KeyCode::Pause => (),
        KeyCode::Menu => (),
        KeyCode::KeypadBegin => (),
        KeyCode::Media(_) => (),
        KeyCode::Modifier(_) => (),
    }
}

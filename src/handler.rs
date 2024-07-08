use crate::app::{App, AppStatus};
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
        KeyCode::Backspace => {}
        KeyCode::Enter => {}
        KeyCode::Left => {}
        KeyCode::Right => {}
        KeyCode::Up => {}
        KeyCode::Down => {}
        KeyCode::Home => {}
        KeyCode::End => {}
        KeyCode::PageUp => {}
        KeyCode::PageDown => {}
        KeyCode::Tab => {}
        KeyCode::BackTab => {}
        KeyCode::Delete => {}
        KeyCode::Insert => {}
        KeyCode::F(_) => {}
        KeyCode::Char(char) => match char {
            'q' => app.set_status(AppStatus::Quitting),
            _ => {}
        },
        KeyCode::Null => {}
        KeyCode::Esc => {}
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

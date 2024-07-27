use crate::app::{App, AppStatus, Focus};
use core::panic;
use std::io::Result;

use ratatui::crossterm::event::{self, Event, KeyCode};

#[derive(PartialEq, Debug, Clone)]
pub enum Action {
    /// Quit the application
    Quit,
    ChangeFocus,
    /// When focusing a list ot items
    NextItem,
    PrevItem,
    /// Show the tasks of the selected area
    NewArea,
    SelectArea,
    NewTask,
    CheckTask,
    ShowInput,
    AcceptInput,
    EscInput,
    /// When writing in the input
    AddChar(char),
    RmChar,
}

pub fn event(app: &mut App) -> Result<Option<Action>> {
    if event::poll(std::time::Duration::from_millis(16))? {
        if let Event::Key(key) = event::read()? {
            if key.kind == event::KeyEventKind::Press {
                return Ok(handle_key(key.code, app));
            }
        }
    }

    Ok(None)
}

fn handle_key(key: KeyCode, app: &mut App) -> Option<Action> {
    match key {
        KeyCode::Tab => {
            if app.focus != Focus::Input {
                return Some(Action::ChangeFocus);
            }
        }
        KeyCode::Backspace => {
            if app.focus == Focus::Input {
                return Some(Action::RmChar);
            }
        }
        KeyCode::Enter => match app.focus {
            Focus::Areas => return Some(Action::SelectArea),
            Focus::Tasks => (),
            Focus::Input => return Some(Action::AcceptInput),
        },
        KeyCode::Char(c) => {
            if app.focus == Focus::Input {
                return Some(Action::AddChar(c));
            }
            match c {
                'q' => return Some(Action::Quit),
                'n' => return Some(Action::ShowInput),
                'j' => match app.focus {
                    Focus::Areas => return Some(Action::NextItem),
                    Focus::Tasks => (),
                    _ => (),
                },
                'k' => match app.focus {
                    Focus::Areas => return Some(Action::PrevItem),
                    Focus::Tasks => (),
                    _ => (),
                },
                _ => (),
            }
        }
        KeyCode::Esc => {
            if app.focus == Focus::Input {
                return Some(Action::EscInput);
            }
        }
        _ => (),
    }

    None
}

pub fn update(app: &mut App, action: Action) -> Result<Option<Action>> {
    // TODO: make an Area and a Task Component
    match (&app.focus, action.clone()) {
        (Focus::Areas, Action::Quit) => app.status = AppStatus::Quitting,
        (Focus::Areas, Action::ChangeFocus) => app.focus = Focus::Tasks,
        (Focus::Areas, Action::NextItem) => app.next_area(),
        (Focus::Areas, Action::PrevItem) => app.prev_area(),
        (Focus::Areas, Action::NewArea) => app.new_area(),
        (Focus::Areas, Action::SelectArea) => app.update_current_area(),
        (Focus::Areas, Action::ShowInput) => {
            app.save_current_pane();
            app.focus = Focus::Input;
        }
        (Focus::Tasks, Action::Quit) => app.status = AppStatus::Quitting,
        (Focus::Tasks, Action::ChangeFocus) => app.focus = Focus::Areas,
        (Focus::Tasks, Action::NextItem) => todo!("Cannot make action NextItem in Tasks"),
        (Focus::Tasks, Action::PrevItem) => todo!("Cannot make action PrevItem in Tasks"),
        (Focus::Tasks, Action::NewTask) => todo!("Cannot make action NewTask in Tasks"),
        (Focus::Tasks, Action::CheckTask) => todo!("Cannot make action CheckTask in Tasks"),
        (Focus::Tasks, Action::ShowInput) => {
            app.save_current_pane();
            app.focus = Focus::Input;
        }
        (Focus::Input, Action::AcceptInput) => {
            app.handle_input_text();
            app.set_prev_pane();
        }
        (Focus::Input, Action::EscInput) => {
            app.input.clear();
            app.set_prev_pane();
        }
        (Focus::Input, Action::AddChar(c)) => app.input.insert_char(c),
        (Focus::Input, Action::RmChar) => app.input.remove_char(),
        _ => panic!("Cannot make {:?} in {:?}", action, app.focus),
    }

    Ok(None)
}

use crate::{
    app::{App, AppStatus, Focus},
    frames::Component,
};
use core::panic;
use ratatui::crossterm::event::{self, Event, KeyCode};
use std::io::Result;

#[derive(PartialEq, Debug, Clone)]
pub enum Action {
    /// Quit the application
    Quit,
    ChangeFocus,
    /// When focusing a list ot items
    NextItem,
    PrevItem,
    /// Show the tasks of the selected area
    NewArea(String),
    NewTask(String),
    CheckTask,
    ShowInput,
    AcceptInput,
    EscInput,
    /// When writing in the input
    AddChar(char),
    RmChar,
    /// No action
    None,
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
            Focus::Areas => (),
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
    match action {
        Action::Quit => {
            app.status = AppStatus::Quitting;
            return Ok(None);
        }
        Action::ChangeFocus => {
            app.focus = match app.focus {
                Focus::Areas => Focus::Tasks,
                Focus::Tasks => Focus::Areas,
                Focus::Input => {
                    app.input.clear();

                    // Sets the focus to the previous pane
                    app.prev_focus.clone().unwrap()
                }
            };
            return Ok(None);
        }
        Action::ShowInput => {
            // Saves the current focus
            app.prev_focus = Some(app.focus.clone());

            app.focus = Focus::Input;
            return Ok(None);
        }
        _ => (),
    }

    match (&app.focus, action.clone()) {
        (Focus::Areas, _) => return Ok(app.areas.handle_action(action)),
        (Focus::Tasks, Action::NextItem) => todo!("Cannot make action NextItem in Tasks"),
        (Focus::Tasks, Action::PrevItem) => todo!("Cannot make action PrevItem in Tasks"),
        (Focus::Tasks, Action::CheckTask) => todo!("Cannot make action CheckTask in Tasks"),
        (Focus::Input, Action::AcceptInput) => {
            if let Some(focus) = &app.prev_focus {
                if *focus == Focus::Areas {
                    return Ok(Some(Action::NewArea(app.input.text.trim().to_string())));
                } else if *focus == Focus::Tasks {
                    return Ok(Some(Action::NewTask(app.input.text.trim().to_string())));
                }
            } else {
                panic!("There should be an app.prevfocus")
            }
        }
        (Focus::Input, Action::EscInput) => {
            app.input.clear();
            // Sets the focus to the previous pane
            app.focus = app.prev_focus.clone().unwrap();
        }
        (Focus::Input, Action::AddChar(c)) => app.input.insert_char(c),
        (Focus::Input, Action::RmChar) => app.input.remove_char(),
        (Focus::Input, Action::NewArea(name)) => {
            app.areas.new_area(&name);
            return Ok(Some(Action::ChangeFocus));
        }
        (Focus::Input, Action::NewTask(desc)) => {
            todo!("Cannot make action NewTask({}) in Input", desc)
        }
        _ => panic!("Cannot make {:?} in {:?}", action, app.focus),
    }

    Ok(None)
}

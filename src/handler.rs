use crate::{
    app::{App, AppStatus, Focus},
    frames::FrameTrait,
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
    match app.focus {
        Focus::Areas => return app.areas.handle_key(key),
        Focus::Input => return app.input.handle_key(key),
        _ => (),
    }

    // TODO: Handle keys for the tasks component
    match key {
        KeyCode::Tab => {
            if app.focus != Focus::Input {
                return Some(Action::ChangeFocus);
            }
        }
        KeyCode::Enter => match app.focus {
            Focus::Tasks => (),
            _ => panic!("Should not be able to get here"),
        },
        KeyCode::Char(c) => {
            match c {
                'q' => return Some(Action::Quit),
                'n' => return Some(Action::ShowInput),
                'j' => match app.focus {
                    Focus::Tasks => (), // TODO: Implement next task
                    _ => panic!("Should not be able to get here"),
                },
                'k' => match app.focus {
                    Focus::Tasks => (), // TODO: Implement prev task
                    _ => panic!("Should not be able to get here"),
                },
                _ => (),
            }
        }
        KeyCode::Esc => {
            // TODO: Handle the escape key for the tasks component
        }
        _ => (),
    }

    None
}

pub fn update(app: &mut App, action: Action) -> Result<Option<Action>> {
    // Handle window level actions. Not component specific.
    match action {
        Action::Quit => {
            app.status = AppStatus::Quitting;
            return Ok(None);
        }
        Action::ChangeFocus => {
            app.focus = match app.focus {
                Focus::Areas => Focus::Tasks,
                Focus::Tasks => Focus::Areas,
                Focus::Input => app.prev_focus.clone().unwrap(),
            };
            return Ok(None);
        }
        Action::ShowInput => {
            // Saves the current focus
            app.prev_focus = Some(app.focus.clone());

            app.focus = Focus::Input;
            return Ok(None);
        }
        Action::AcceptInput => {
            // Get the text from the input
            let text = app.input.text.trim().to_string();
            app.input.clear();

            // Change focus to the previous focus in order to make the required component handle
            // the action
            update(app, Action::ChangeFocus)?;

            return match app.focus {
                Focus::Areas => Ok(Some(Action::NewArea(text))),
                Focus::Tasks => Ok(Some(Action::NewTask(text))),
                _ => panic!("Unexpected focus"),
            };
        }
        _ => (),
    }

    let new_action = match &app.focus {
        Focus::Areas => app.areas.handle_action(action),
        //(Focus::Tasks, Action::NextItem) => todo!("Cannot make action NextItem in Tasks"),
        //(Focus::Tasks, Action::PrevItem) => todo!("Cannot make action PrevItem in Tasks"),
        //(Focus::Tasks, Action::CheckTask) => todo!("Cannot make action CheckTask in Tasks"),
        Focus::Input => app.input.handle_action(action),
        _ => panic!("Unespected focus"),
    };

    Ok(new_action)
}

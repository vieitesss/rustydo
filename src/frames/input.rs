use super::FrameTrait;
use crate::handler::Action;
use ratatui::{
    crossterm::event::KeyCode,
    layout::{Constraint, Direction, Layout, Rect},
    style::{Color, Style},
    widgets::{Block, Borders, Clear, Paragraph, Widget},
    Frame,
};
use std::cmp::min;

pub struct Input {
    pub text: String,
    text_bounds: (u16, u16),
    text_pos: u16,
}

impl Default for Input {
    fn default() -> Self {
        Input {
            text: String::new(),
            text_bounds: (0, 0),
            text_pos: 0,
        }
    }
}

impl FrameTrait for Input {
    fn render(&mut self, frame: &mut Frame, _rect: Option<Rect>) {
        let frame_height = frame.size().height;
        let [_, input_vert, _] = Layout::default()
            .direction(Direction::Vertical)
            .constraints(vec![
                Constraint::Length(frame_height / 2 - 3),
                Constraint::Length(3),
                Constraint::Length(frame_height / 2 - 3),
            ])
            .areas(frame.size());

        let [_, input_area, _] = Layout::default()
            .direction(Direction::Horizontal)
            .constraints(vec![
                Constraint::Min(1),
                Constraint::Max(40),
                Constraint::Min(1),
            ])
            .areas(input_vert);

        Clear.render(input_area, frame.buffer_mut());

        let input_block = Block::default()
            .title("Input")
            .borders(Borders::ALL)
            .border_style(Style::default().fg(Color::Yellow));

        let inner_area = input_block.inner(input_area);
        self.text_bounds = (inner_area.left() + 1, inner_area.right());

        frame.render_widget(input_block, input_area);

        // Set the cursor in the correct position
        let cursor_pos = min(self.text_max_len(), self.text_pos);
        frame.set_cursor(inner_area.left() + cursor_pos, inner_area.top());

        // Render the text inside the input box
        let maxl = self.text_max_len() as usize;
        let textl = self.text.len();
        let mut line: String = self.text.clone();
        if textl >= maxl {
            line = self.text.chars().skip(textl - maxl).take(maxl).collect();
        }

        let paragraph = Paragraph::new(line);

        frame.render_widget(paragraph, inner_area)
    }

    fn handle_key(&mut self, key: KeyCode) -> Option<Action> {
        match key {
            KeyCode::Backspace => Some(Action::RmChar),
            KeyCode::Enter => Some(Action::AcceptInput),
            KeyCode::Char(c) => Some(Action::AddChar(c)),
            KeyCode::Esc => Some(Action::EscInput),
            _ => None,
        }
    }

    fn handle_action(&mut self, action: Action) -> Option<Action> {
        match action {
            Action::EscInput => {
                self.clear();

                Some(Action::ChangeFocus)
            }
            Action::AddChar(c) => {
                self.insert_char(c);
                None
            }
            Action::RmChar => {
                self.remove_char();
                None
            }
            //Action::NewTask(desc) => {
            //    todo!("Cannot make action NewTask({}) in Input", desc)
            //}
            _ => panic!("Cannot handle action {:?} in Input", action),
        }
    }
}

impl Input {
    pub fn increase_text_pos(&mut self) {
        self.text_pos += 1;
    }

    pub fn decrease_text_pos(&mut self) {
        self.text_pos -= 1;
    }

    pub fn text_max_len(&mut self) -> u16 {
        self.text_bounds.1 - self.text_bounds.0
    }

    pub fn clear(&mut self) {
        self.text = String::new();
        self.text_pos = 0;
    }

    pub fn insert_char(&mut self, c: char) {
        self.text += &c.to_string();
        self.text_pos += 1;
    }

    pub fn remove_char(&mut self) {
        if self.text_pos > 0 {
            self.decrease_text_pos();
        }

        let chars = self.text.chars();
        let lhs: String = chars.clone().take(self.text_pos as usize).collect();
        let rhs: String = chars
            .clone()
            .skip(self.text_pos as usize + 1)
            .take(self.text.len() - self.text_pos as usize)
            .collect();

        self.text = String::from(format!("{}{}", lhs, rhs));
    }
}

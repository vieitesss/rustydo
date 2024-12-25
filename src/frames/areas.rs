use super::FrameTrait;
use crate::{
    handler::Action,
    model::{area::Area, task::Task},
};
use ratatui::{
    crossterm::event::KeyCode,
    layout::Rect,
    style::{Color, Style},
    text::{Line, Text},
    widgets::{Block, Borders, List, ListItem, ListState},
    Frame,
};

const NORMAL: Style = Style::new().fg(Color::White);
const SELECTED: Style = Style::new().fg(Color::Yellow);

#[derive(Debug, Clone, PartialEq)]
pub struct Areas {
    pub list: Vec<Area>,
    pub state: ListState,
}

impl Default for Areas {
    fn default() -> Self {
        let mut uni = Area::new("Universidad");
        uni.selected = true;
        uni.tasks.push(Task::new("mandar correo", *uni.id()));
        let mut casa = Area::new("Casa");
        casa.tasks
            .push(Task::new("recoger la habitación", *casa.id()));
        casa.tasks.push(Task::new("hacer la maleta", *uni.id()));
        casa.tasks.get_mut(1).unwrap().done = true;

        // Select the first area
        let mut state: ListState = ListState::default();
        state.select(Some(0));

        Areas {
            list: vec![uni, casa],
            state,
        }
    }
}

impl FrameTrait for Areas {
    fn render(&mut self, frame: &mut Frame, rect: Option<Rect>) {
        assert!(rect.is_some());

        let inner = Block::bordered().inner(rect.unwrap());

        let mut areas_items = Vec::<ListItem>::new();
        for area in self.list.iter() {
            areas_items.push(ListItem::from(area));
        }

        let areas_block = Block::new().borders(Borders::NONE);
        let areas = List::new(areas_items).block(areas_block);

        frame.render_widget(areas, inner);
    }

    fn handle_key(&mut self, key: KeyCode) -> Option<Action> {
        match key {
            KeyCode::Tab => Some(Action::ChangeFocus),
            KeyCode::Char(c) => match c {
                'q' => Some(Action::Quit),
                'n' => Some(Action::ShowInput),
                'j' => Some(Action::NextItem),
                'k' => Some(Action::PrevItem),
                _ => None,
            },
            _ => None,
        }
    }

    fn handle_action(&mut self, action: Action) -> Option<Action> {
        match action {
            Action::NextItem => {
                self.next_area();
                None
            }
            Action::PrevItem => {
                self.prev_area();
                None
            }
            Action::NewArea(name) => {
                self.new_area(&name);
                None
            }
            _ => panic!("Cannot handle action {:?} in Input", action),
        }
    }
}

impl Areas {
    pub fn new_area(&mut self, name: &str) {
        let area = Area::new(name);
        self.list.push(area);
    }

    pub fn next_area(&mut self) {
        if let Some(selected) = self.state.selected() {
            self.list[selected].selected = false;
            let next = (selected + 1) % self.list.len();
            self.list[next].selected = true;
        } else {
            if !self.list.is_empty() {
                self.list[0].selected = true;
            }
        }
        self.state.select_next();
    }

    pub fn prev_area(&mut self) {
        let list_len = self.list.len();
        if let Some(selected) = self.state.selected() {
            self.list[selected].selected = false;
            let prev = if selected == 0 {
                list_len - 1
            } else {
                selected - 1
            };
            self.list[prev].selected = true;
        } else {
            if !self.list.is_empty() {
                self.list[list_len - 1].selected = true;
            }
        }
        self.state.select_previous();
    }
}

impl From<&Area> for ListItem<'_> {
    fn from(area: &Area) -> Self {
        let area_text = if area.selected {
            Text::from(Line::from(format!("> {}", area.title)).style(SELECTED))
        } else {
            Text::from(Line::from(format!("  {}", area.title)).style(NORMAL))
        };

        ListItem::new(area_text)
    }
}

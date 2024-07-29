use ratatui::{
    layout::Rect,
    style::Stylize,
    text::{Line, Text},
    widgets::{Block, Borders, List, ListItem}, Frame,
};

use crate::model::{area::Area, task::Task};

use super::Component;

pub struct Areas {
    pub list: Vec<Area>,
    /// The area whose tasks are shown
    pub current_area: usize,
    /// The area selected in the list
    pub selected_area: usize,
}

impl Default for Areas {
    fn default() -> Self {
        let mut uni = Area::new("Universidad");
        uni.tasks.push(Task::new("mandar correo", *uni.id()));
        let mut casa = Area::new("Casa");
        casa.tasks
            .push(Task::new("recoger la habitación", *casa.id()));
        casa.tasks.push(Task::new("hacer la maleta", *uni.id()));
        casa.tasks.get_mut(1).unwrap().done = true;
        Areas {
            list: vec![uni, casa],
            current_area: 0,
            selected_area: 0,
        }
    }
}

impl Component for Areas {
    fn render(&mut self, frame: &mut Frame, rect: Option<Rect>) {
        assert!(rect.is_some());

        let inner = Block::bordered().inner(rect.unwrap());

        let mut areas_items = Vec::<ListItem>::new();
        for (index, area) in self.list.iter().enumerate() {
            let mut title = Text::from(Line::from(format!("  {}", area.title.clone())).bold());
            if index == self.selected_area {
                title = Text::from(Line::from(format!("> {}", area.title.clone()).bold()));
            }
            if index == self.current_area {
                title = title.clone().yellow()
            }
            areas_items.push(ListItem::new(title));
        }

        let areas_block = Block::new().borders(Borders::NONE);
        let areas = List::new(areas_items).block(areas_block);

        frame.render_widget(areas, inner);
    }
}

impl Areas {
    pub fn new_area(&mut self, name: &str) {
        let area = Area::new(name);
        self.list.push(area);
    }

    pub fn next_area(&mut self) {
        self.selected_area = (self.selected_area as usize + 1) % self.list.len()
    }

    pub fn prev_area(&mut self) {
        if self.selected_area as usize == 0 {
            self.selected_area = self.list.len() - 1;
        } else {
            self.selected_area -= 1;
        }
    }

    pub fn update_current_area(&mut self) {
        self.current_area = self.selected_area
    }
}

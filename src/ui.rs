#![allow(unused_imports)]

use crate::app::App;

use crate::app::AppWindow;
use ratatui::{
    layout::{Constraint, Direction, Layout, Rect},
    style::{Color, Style, Stylize},
    text::Line,
    widgets::{Block, Borders, List, ListItem, Padding, Paragraph, Wrap},
    Frame,
};

pub fn render(frame: &mut Frame, app: &mut App) {
    match app.get_window() {
        AppWindow::Main => render_main(frame, app),
    }
}

fn render_main(frame: &mut Frame, app: &mut App) {
    let chunks = Layout::default()
        .direction(Direction::Horizontal)
        .constraints(vec![Constraint::Percentage(30), Constraint::Percentage(70)])
        .split(frame.size());

    let areas_block = Block::default().title("Areas").borders(Borders::ALL);

    let tasks_block = Block::default().title("Tasks").borders(Borders::ALL);

    frame.render_widget(areas_block, chunks[0]);
    frame.render_widget(tasks_block, chunks[1]);

    render_tasks(frame, chunks[1], app);
}

fn render_tasks(frame: &mut Frame, rect: Rect, app: &mut App) {
    // Create inner area of the tasks_area
    let inner = Block::bordered().inner(rect);

    // Layout for the inner area
    let tasks_chunks = Layout::default()
        .direction(Direction::Vertical)
        .constraints(vec![Constraint::Length(3), Constraint::Fill(0)])
        .split(inner);

    // Centered, without borders, padding-top 1 and a fixed height of 3
    // It is going to be centered vertically and horizontally
    let title_block = Block::new().borders(Borders::NONE).padding(Padding::top(1));
    let title = Paragraph::new("Universidad")
        .bold()
        .centered()
        .wrap(Wrap { trim: true })
        .block(title_block);

    let mut tasks_items = Vec::<ListItem>::new();
    if let Some(mut area) = app.get_current_area() {
        for task in area.get_tasks() {
            let check = if task.is_done() { "󰱒" } else { "󰄱" };
            tasks_items.push(ListItem::new(format!("{} {}", check, task.get_title())));
        }
    }

    let tasks_block = Block::new().borders(Borders::NONE);
    let tasks = List::new(tasks_items).block(tasks_block);

    frame.render_widget(title, tasks_chunks[0]);
    frame.render_widget(tasks, tasks_chunks[1]);
}

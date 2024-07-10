use std::cmp::min;

use crate::app::{App, AppWindow, WindowPane};

use ratatui::{
    layout::{Constraint, Direction, Layout, Rect},
    style::{Color, Style, Stylize},
    text::{Line, Text},
    widgets::{Block, Borders, Clear, List, ListItem, Padding, Paragraph, Widget, Wrap},
    Frame,
};

pub fn render(frame: &mut Frame, app: &mut App) {
    match app.get_window() {
        AppWindow::Main => render_main(frame, app),
    }
}

fn render_main(frame: &mut Frame, app: &mut App) {
    let main_help = Layout::default()
        .direction(Direction::Vertical)
        .constraints(vec![Constraint::Fill(0), Constraint::Length(1)])
        .split(frame.size());

    let areas_tasks = Layout::default()
        .direction(Direction::Horizontal)
        .constraints(vec![Constraint::Percentage(30), Constraint::Percentage(70)])
        .split(main_help[0]);

    let mut areas_block = Block::default().title("Areas").borders(Borders::ALL);
    let mut tasks_block = Block::default().title("Tasks").borders(Borders::ALL);

    let current_pane = app.get_pane();
    match current_pane {
        WindowPane::Areas => {
            areas_block = areas_block
                .clone()
                .border_style(Style::default().fg(Color::Yellow));
        }
        WindowPane::Tasks => {
            tasks_block = tasks_block
                .clone()
                .border_style(Style::default().fg(Color::Yellow));
        }
        WindowPane::Input => {}
    }

    frame.render_widget(areas_block, areas_tasks[0]);
    frame.render_widget(tasks_block, areas_tasks[1]);

    render_help(frame, main_help[1], app);
    render_areas(frame, areas_tasks[0], app);
    render_tasks(frame, areas_tasks[1], app);

    if current_pane == WindowPane::Input {
        render_input(frame, app);
    }
}

fn render_help(frame: &mut Frame, rect: Rect, app: &mut App) {
    let mut help_text = String::new();
    if app.get_pane() == WindowPane::Input {
        help_text += "exit [esc]  accept [enter]";
    } else {
        help_text += "help [?]  exit [q]  focus next [tab]";
    }

    let help = match app.get_window() {
        AppWindow::Main => {
            match app.get_pane() {
                WindowPane::Areas => help_text += "  new area [n]",
                WindowPane::Tasks => help_text += "  new task [n]",
                WindowPane::Input => {}
            }
            Paragraph::new(help_text).block(Block::default().borders(Borders::NONE))
        }
    };

    frame.render_widget(help, rect);
}

fn render_areas(frame: &mut Frame, rect: Rect, app: &mut App) {
    let inner = Block::bordered().inner(rect);

    let mut areas_items = Vec::<ListItem>::new();
    for area in app.get_areas() {
        let title = Text::from(Line::from(area.get_title()).bold());
        areas_items.push(ListItem::new(title));
    }

    let areas_block = Block::new().borders(Borders::NONE);
    let areas = List::new(areas_items).block(areas_block);

    frame.render_widget(areas, inner);
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

fn render_input(frame: &mut Frame, app: &mut App) {
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
    app.set_input_text_bounds(inner_area.left() + 1, inner_area.right());

    frame.render_widget(input_block, input_area);

    // Set the cursor in the correct position
    let cursor_pos = min(app.get_input_text_max_len(), app.get_input_text_pos());
    frame.set_cursor(inner_area.left() + cursor_pos, inner_area.top());

    // Render the text inside the input box
    let mut input_text = app.get_input_text();
    let input_text_len = input_text.len();
    let input_text_max_len = app.get_input_text_max_len() as usize;
    if input_text_len >= input_text_max_len {
        input_text = input_text
            .chars()
            .skip(input_text_len - input_text_max_len)
            .take(input_text_max_len)
            .collect();
    };
    let text = Paragraph::new(input_text);

    frame.render_widget(text, inner_area);
}

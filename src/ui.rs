use crate::{
    app::{App, AppWindow, Focus},
    components::Component,
};

use ratatui::{
    layout::{Constraint, Direction, Layout, Rect},
    style::{Color, Style, Stylize},
    widgets::{Block, Borders, List, ListItem, Padding, Paragraph, Wrap},
    Frame,
};

pub fn render(frame: &mut Frame, app: &mut App) {
    match app.window {
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

    match &app.focus {
        Focus::Areas => {
            areas_block = areas_block
                .clone()
                .border_style(Style::default().fg(Color::Yellow));
        }
        Focus::Tasks => {
            tasks_block = tasks_block
                .clone()
                .border_style(Style::default().fg(Color::Yellow));
        }
        Focus::Input => {}
    }

    frame.render_widget(areas_block, areas_tasks[0]);
    frame.render_widget(tasks_block, areas_tasks[1]);

    render_help(frame, main_help[1], app);
    app.areas.render(frame, Some(areas_tasks[0]));
    render_tasks(frame, areas_tasks[1], app);

    if app.focus == Focus::Input {
        app.input.render(frame, None);
    }
}

fn render_help(frame: &mut Frame, rect: Rect, app: &mut App) {
    let mut help_text = String::new();
    if app.focus == Focus::Input {
        help_text += "exit [esc]  accept [enter]";
    } else {
        help_text += "help [?]  exit [q]  focus next [tab]";
    }

    let help = match app.window {
        AppWindow::Main => {
            match app.focus {
                Focus::Areas => help_text += "  new area [n]  / [j/k]  select [enter]",
                Focus::Tasks => help_text += "  new task [n]",
                Focus::Input => {}
            }
            Paragraph::new(help_text).block(Block::default().borders(Borders::NONE))
        }
    };

    frame.render_widget(help, rect);
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
    let title = Paragraph::new(app.areas.list[app.areas.current_area].title.clone())
        .bold()
        .centered()
        .wrap(Wrap { trim: true })
        .block(title_block);

    let mut tasks_items = Vec::<ListItem>::new();
    for task in &app.areas.list[app.areas.current_area].tasks {
        let check = if task.done { "󰱒" } else { "󰄱" };
        tasks_items.push(ListItem::new(format!("{} {}", check, task.title)));
    }

    let tasks_block = Block::new().borders(Borders::NONE);
    let tasks = List::new(tasks_items).block(tasks_block);

    frame.render_widget(title, tasks_chunks[0]);
    frame.render_widget(tasks, tasks_chunks[1]);
}

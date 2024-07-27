use std::io::{self, stdout, Result};
use std::panic;

use ratatui::{
    backend::CrosstermBackend,
    crossterm::terminal::{self, EnterAlternateScreen, LeaveAlternateScreen},
    prelude::Backend,
    Terminal,
};

use rustydo::app::App;
use rustydo::handler;
use rustydo::ui;

fn init() -> Result<Terminal<impl Backend>> {
    ratatui::crossterm::execute!(io::stderr(), EnterAlternateScreen)?;
    terminal::enable_raw_mode()?;

    // Define a custom panic hook to reset the terminal properties
    let panic_hook = panic::take_hook();
    panic::set_hook(Box::new(move |panic| {
        quit().expect("error quiting the app");
        panic_hook(panic);
    }));

    let mut terminal = Terminal::new(CrosstermBackend::new(stdout()))?;
    terminal.clear()?;

    Ok(terminal)
}

fn quit() -> Result<()> {
    ratatui::crossterm::execute!(io::stderr(), LeaveAlternateScreen)?;
    terminal::disable_raw_mode()?;

    Ok(())
}

fn main() -> Result<()> {
    let mut terminal = init()?;
    let mut app = App::new();

    while app.is_running() {
        terminal.draw(|frame| ui::render(frame, &mut app))?;
        let mut action = handler::event(&mut app)?;

        while action.is_some() {
            action = handler::update(&mut app, action.unwrap())?;
        }
    }

    quit()?;

    Ok(())
}

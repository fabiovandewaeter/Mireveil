use std::io::stdout;

use app::{App, Config};
use color_eyre::Result;
use crossterm::{event::EnableMouseCapture, execute};

mod app;
mod game_objects;
mod map;
mod systems;

fn main() -> Result<()> {
    color_eyre::install()?;
    let terminal = ratatui::init();
    execute!(stdout(), EnableMouseCapture)?;
    let config = Config::default();
    let app = App::new(config);
    let app_result = app.run(terminal);
    ratatui::restore();
    app_result
}

use std::io::stdout;

use app::{App, Config};
use color_eyre::Result;
use crossterm::{
    event::{DisableMouseCapture, EnableMouseCapture},
    execute,
};

mod actions;
mod app;
mod common;
mod entities;
mod items;
mod map;
mod menu;
mod structures;
mod systems;

/// disables mouse capture even if app crash
struct MouseGuard;

impl MouseGuard {
    fn new() -> Result<Self> {
        execute!(stdout(), EnableMouseCapture)?;
        Ok(Self)
    }
}

impl Drop for MouseGuard {
    fn drop(&mut self) {
        if let Err(e) = execute!(stdout(), DisableMouseCapture) {
            eprintln!("Failed to disable mouse capture: {}", e);
        }
    }
}

fn main() -> Result<()> {
    color_eyre::install()?;

    let terminal = ratatui::init();
    let _mouse_guard = MouseGuard::new()?;
    let config = Config::default();
    let app = App::new(config);
    let app_result = app.run(terminal);

    ratatui::restore();
    app_result
}

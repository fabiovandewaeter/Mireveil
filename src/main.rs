use app::{App, Config};
use color_eyre::Result;

mod app;
mod game_objects;
mod map;
mod systems;

fn main() -> Result<()> {
    color_eyre::install()?;
    let terminal = ratatui::init();
    let config = Config::default();
    let app = App::new(config);
    let app_result = app.run(terminal);
    ratatui::restore();
    app_result
}

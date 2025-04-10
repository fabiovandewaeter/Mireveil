use app::{App, Config};
use color_eyre::Result;

mod app;
mod entities;
mod map;

fn main() -> Result<()> {
    color_eyre::install()?;
    let terminal = ratatui::init();
    let config = Config::default();
    let app = App::new(config);
    let app_result = app.run(terminal);
    ratatui::restore();
    app_result
}

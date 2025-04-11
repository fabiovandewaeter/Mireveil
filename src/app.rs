use color_eyre::Result;
use crossterm::event::{self, Event, KeyCode, KeyEvent, KeyEventKind};
use ratatui::{
    Terminal,
    backend::CrosstermBackend,
    prelude::*,
    widgets::{Block, Borders},
};

use crate::{map::map::*, systems::entity_manager::EntityManager};

#[derive(Clone)]
pub struct Config {
    background_style: Style,
}

impl Default for Config {
    fn default() -> Self {
        Self {
            background_style: Style::default().bg(Color::Rgb(131, 105, 83)),
        }
    }
}

pub struct App {
    map: Map,
    entity_manager: EntityManager,
    config: Config,
    exit: bool,
}

impl App {
    pub fn new(config: Config) -> Self {
        Self {
            map: Map::default(),
            entity_manager: EntityManager::new(),
            config,
            exit: false,
        }
    }

    pub fn run(mut self, mut terminal: Terminal<CrosstermBackend<std::io::Stdout>>) -> Result<()> {
        while !self.exit {
            terminal.draw(|f| self.draw(f))?;
            self.handle_events()?;
        }
        Ok(())
    }

    fn handle_events(&mut self) -> Result<()> {
        if let Event::Key(key) = event::read()? {
            if key.kind == KeyEventKind::Press {
                self.process_key(key);
            }
        }
        Ok(())
    }

    fn process_key(&mut self, key: KeyEvent) {
        if key.kind == KeyEventKind::Press {
            match key.code {
                KeyCode::Char('q') => self.exit = true,
                //_ => self.player.process_key(key.code, &mut self.map),
                _ => self.entity_manager.process_key(key.code, &mut self.map),
            }
        }
    }

    fn draw(&self, frame: &mut Frame) {
        let area = frame.area();

        // draw background color
        let background = Block::default()
            .style(self.config.background_style)
            .borders(Borders::NONE);
        frame.render_widget(background, area);

        //let (camera_x, camera_y) = self.player.calculate_camera(area);
        let buffer = frame.buffer_mut();

        // draw map
        let (camera_x, camera_y) = self.entity_manager.player.calculate_camera(area);
        self.map.draw(buffer, area, camera_x, camera_y);

        // draw entities
        self.entity_manager.draw(buffer, area);
    }
}

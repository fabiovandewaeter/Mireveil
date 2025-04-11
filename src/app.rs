use color_eyre::Result;
use crossterm::event::{self, Event, KeyCode, KeyEvent, KeyEventKind};
use ratatui::{
    Terminal,
    backend::CrosstermBackend,
    prelude::*,
    widgets::{Block, Borders},
};

use crate::entities::player::*;
use crate::map::map::*;

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
    player: Player,
    map: Map,
    config: Config,
    exit: bool,
}

impl App {
    pub fn new(config: Config) -> Self {
        let map = Map::default();
        let app = Self {
            player: Player::new((map.size.0 / 2, map.size.1 / 2)),
            map,
            config,
            exit: false,
        };
        app
    }

    pub fn run(mut self, mut terminal: Terminal<CrosstermBackend<std::io::Stdout>>) -> Result<()> {
        while !self.exit {
            terminal.draw(|f| self.draw(f))?;
            self.handle_events()?;
        }
        Ok(())
    }

    fn move_player(&mut self, dx: i32, dy: i32) {
        let new_x = self.player.position.0 + dx;
        let new_y = self.player.position.1 + dy;

        if let Some(tile) = self.map.get_tile(new_x, new_y) {
            if !tile.solid {
                self.player.position = (new_x, new_y);
                self.map.load_around((
                    new_x.div_euclid(CHUNK_SIZE as i32),
                    new_y.div_euclid(CHUNK_SIZE as i32),
                ));
            }
        }
    }

    fn handle_events(&mut self) -> Result<()> {
        // Lire tous les événements en attente
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
                KeyCode::Up => self.move_player(0, -1),
                KeyCode::Down => self.move_player(0, 1),
                KeyCode::Left => self.move_player(-1, 0),
                KeyCode::Right => self.move_player(1, 0),
                _ => {}
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

        let (camera_x, camera_y) = self.player.calculate_camera(area);
        let buffer = frame.buffer_mut();

        // draw map
        for screen_y in 0..area.height {
            for screen_x in 0..area.width {
                let world_x = camera_x + screen_x as i32;
                let world_y = camera_y + screen_y as i32;
                let (symbol, style) = self
                    .map
                    .get_tile(world_x, world_y)
                    .map(|tile| (tile.symbol, tile.style))
                    .unwrap_or(("#", Style::default().fg(Color::Red)));
                let position: Position = Position {
                    x: screen_x,
                    y: screen_y,
                };
                let cell = buffer.cell_mut(position).unwrap();
                cell.set_symbol(symbol);
                cell.set_style(style);
            }
        }

        // Dessiner le joueur
        let position: Position = Position {
            x: area.width / 2,
            y: area.height / 2,
        };
        let player_cell = buffer.cell_mut(position).unwrap();
        player_cell.set_symbol(self.player.symbol);
        player_cell.set_style(self.player.style);
    }
}

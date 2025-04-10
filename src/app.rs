use color_eyre::Result;
use crossterm::event::{self, Event, KeyCode, KeyEvent, KeyEventKind};
use ratatui::{
    Terminal,
    backend::CrosstermBackend,
    prelude::*,
    widgets::{Block, Borders},
};

use crate::entities::player::*;
use crate::map::*;

#[derive(Clone)]
pub struct Config {
    wall_char: &'static str,
    floor_char: &'static str,
    wall_style: Style,
    floor_style: Style,
    background_style: Style,
}

impl Default for Config {
    fn default() -> Self {
        Self {
            wall_char: "#",
            floor_char: "·",
            wall_style: Style::default().fg(Color::White),
            floor_style: Style::default().fg(Color::Rgb(50, 50, 50)),
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

    fn move_player(&mut self, dx: i16, dy: i16) {
        let new_x = self.player.position.0 as i16 + dx;
        let new_y = self.player.position.1 as i16 + dy;

        if new_x >= 0
            && new_x < self.map.size.0 as i16
            && new_y >= 0
            && new_y < self.map.size.1 as i16
            && !self.map.cells[new_y as usize][new_x as usize]
        {
            self.player.position = (new_x as u16, new_y as u16);
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
        // Créez un Block avec la couleur d'arrière-plan souhaitée
        let background = Block::default()
            .style(self.config.background_style)
            .borders(Borders::NONE); // vous pouvez choisir d'afficher ou non des bordures
        // Le widget de fond est dessiné avant les autres widgets pour couvrir toute la zone
        frame.render_widget(background, area);

        let (camera_x, camera_y) = self.player.calculate_camera(area);
        let buffer = frame.buffer_mut();

        // Dessiner la carte
        for screen_y in 0..area.height {
            for screen_x in 0..area.width {
                let world_x = camera_x + screen_x as i32;
                let world_y = camera_y + screen_y as i32;
                let (symbol, style) = self.get_tile_representation(world_x, world_y);
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

    fn get_tile_representation(&self, x: i32, y: i32) -> (&'static str, Style) {
        if x < 0 || y < 0 || x >= self.map.size.0 as i32 || y >= self.map.size.1 as i32 {
            (self.config.wall_char, self.config.wall_style)
        } else {
            let is_wall = self.map.cells[y as usize][x as usize];
            if is_wall {
                (self.config.wall_char, self.config.wall_style)
            } else {
                (self.config.floor_char, self.config.floor_style)
            }
        }
    }
}

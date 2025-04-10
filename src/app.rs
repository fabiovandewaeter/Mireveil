use color_eyre::Result;
use crossterm::event::{self, Event, KeyCode, KeyEvent, KeyEventKind};
use ratatui::{Terminal, backend::CrosstermBackend, prelude::*};

#[derive(Clone)]
pub struct Config {
    player_char: &'static str,
    wall_char: &'static str,
    floor_char: &'static str,
    player_style: Style,
    wall_style: Style,
    floor_style: Style,
    map_size: (u16, u16),
}

impl Default for Config {
    fn default() -> Self {
        Self {
            player_char: "@",
            wall_char: "#",
            floor_char: ".",
            player_style: Style::default().fg(Color::Yellow),
            wall_style: Style::default().fg(Color::DarkGray),
            floor_style: Style::default().fg(Color::Rgb(50, 50, 50)),
            map_size: (10000, 10000),
        }
    }
}

pub struct App {
    player_pos: (u16, u16),
    map: Vec<Vec<bool>>,
    config: Config,
    exit: bool,
}

impl App {
    pub fn new(config: Config) -> Self {
        let map = vec![vec![false; config.map_size.0 as usize]; config.map_size.1 as usize];
        let mut app = Self {
            player_pos: (config.map_size.0 / 2, config.map_size.1 / 2),
            map,
            config,
            exit: false,
        };
        app.generate_basic_map();
        app
    }

    fn generate_basic_map(&mut self) {
        for y in 0..self.config.map_size.1 {
            for x in 0..self.config.map_size.0 {
                self.map[y as usize][x as usize] = x == 0
                    || y == 0
                    || x == self.config.map_size.0 - 1
                    || y == self.config.map_size.1 - 1
                    || (x % 10 == 0 && y % 5 == 0);
            }
        }
    }

    fn move_player(&mut self, dx: i16, dy: i16) {
        let new_x = self.player_pos.0 as i16 + dx;
        let new_y = self.player_pos.1 as i16 + dy;

        if new_x >= 0
            && new_x < self.config.map_size.0 as i16
            && new_y >= 0
            && new_y < self.config.map_size.1 as i16
            && !self.map[new_y as usize][new_x as usize]
        {
            self.player_pos = (new_x as u16, new_y as u16);
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

    fn calculate_camera(&self, area: Rect) -> (i32, i32) {
        (
            self.player_pos.0 as i32 - area.width as i32 / 2,
            self.player_pos.1 as i32 - area.height as i32 / 2,
        )
    }

    fn draw(&self, frame: &mut Frame) {
        let config = &self.config;
        let area = frame.area();
        let (camera_x, camera_y) = self.calculate_camera(area);
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
        let center_x = area.width / 2;
        let center_y = area.height / 2;
        let position: Position = Position {
            x: center_x,
            y: center_y,
        };
        let player_cell = buffer.cell_mut(position).unwrap();
        player_cell.set_symbol(config.player_char);
        player_cell.set_style(config.player_style);
    }

    fn get_tile_representation(&self, x: i32, y: i32) -> (&'static str, Style) {
        if x < 0
            || y < 0
            || x >= self.config.map_size.0 as i32
            || y >= self.config.map_size.1 as i32
        {
            (self.config.wall_char, self.config.wall_style)
        } else {
            let is_wall = self.map[y as usize][x as usize];
            if is_wall {
                (self.config.wall_char, self.config.wall_style)
            } else {
                (self.config.floor_char, self.config.floor_style)
            }
        }
    }
}

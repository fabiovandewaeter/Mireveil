// main.rs
use color_eyre::Result;
use crossterm::event::{self, Event, KeyCode, KeyEventKind};
use ratatui::{Terminal, backend::CrosstermBackend, prelude::*, widgets::*};

#[derive(Clone)]
struct Config {
    player_char: char,
    wall_char: char,
    floor_char: char,
    player_style: Style,
    wall_style: Style,
    floor_style: Style,
    map_size: (u16, u16),
}

impl Default for Config {
    fn default() -> Self {
        Self {
            player_char: '@',
            wall_char: '#',
            floor_char: '.',
            player_style: Style::default().fg(Color::Yellow),
            wall_style: Style::default().fg(Color::DarkGray),
            floor_style: Style::default().fg(Color::Rgb(50, 50, 50)),
            map_size: (10000, 10000),
        }
    }
}

struct App {
    player_pos: (u16, u16),
    map: Vec<Vec<bool>>,
    config: Config,
    exit: bool,
}

impl App {
    fn new(config: Config) -> Self {
        let mut map = vec![vec![false; config.map_size.0 as usize]; config.map_size.1 as usize];
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

    fn run(mut self, mut terminal: Terminal<CrosstermBackend<std::io::Stdout>>) -> Result<()> {
        while !self.exit {
            terminal.draw(|f| self.draw(f))?;
            self.handle_events()?;
        }
        Ok(())
    }

    fn handle_events(&mut self) -> Result<()> {
        if let Event::Key(key) = event::read()? {
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
        Ok(())
    }

    fn draw(&self, frame: &mut Frame) {
        let config = &self.config;
        let area = frame.size();

        let player_x = self.player_pos.0 as i32;
        let player_y = self.player_pos.1 as i32;
        let screen_width = area.width as i32;
        let screen_height = area.height as i32;

        // Calcul de la position de la caméra pour centrer le joueur
        let camera_x = player_x - screen_width / 2;
        let camera_y = player_y - screen_height / 2;

        // Dessiner chaque cellule de l'écran
        for screen_y in 0..area.height {
            for screen_x in 0..area.width {
                let world_x = camera_x + screen_x as i32;
                let world_y = camera_y + screen_y as i32;

                // Déterminer le caractère et le style
                let (tile, style) = if world_x < 0
                    || world_y < 0
                    || world_x >= config.map_size.0 as i32
                    || world_y >= config.map_size.1 as i32
                {
                    // Hors de la carte : mur
                    (config.wall_char, config.wall_style)
                } else {
                    // Dans la carte
                    let is_wall = self.map[world_y as usize][world_x as usize];
                    if is_wall {
                        (config.wall_char, config.wall_style)
                    } else {
                        (config.floor_char, config.floor_style)
                    }
                };

                let block = Paragraph::new(tile.to_string()).style(style);
                frame.render_widget(block, Rect::new(screen_x, screen_y, 1, 1));
            }
        }

        // Dessiner le joueur au centre de l'écran
        let player_screen_x = area.width / 2;
        let player_screen_y = area.height / 2;
        let player = Paragraph::new(config.player_char.to_string()).style(config.player_style);
        frame.render_widget(player, Rect::new(player_screen_x, player_screen_y, 1, 1));
    }
}

fn main() -> Result<()> {
    color_eyre::install()?;
    let terminal = ratatui::init();
    let config = Config::default();
    let app = App::new(config);
    let app_result = app.run(terminal);
    ratatui::restore();
    app_result
}

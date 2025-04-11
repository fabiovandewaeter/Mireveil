use crossterm::event::KeyCode;
use ratatui::{
    buffer::Buffer,
    layout::{Position, Rect},
    style::{Color, Style},
};

use crate::map::map::{CHUNK_SIZE, Map};

use super::entity::Drawable;

const SYMBOL: &'static str = "@";
const SYMBOL_COLOR: Color = Color::Yellow;

pub struct Player {
    pub position: (i32, i32),
    symbol: &'static str,
    style: Style,
}

pub trait Playable {
    fn process_key(&mut self, key_code: KeyCode, map: &mut Map);
}

impl Player {
    pub fn new(position: (i32, i32)) -> Self {
        Self {
            position,
            symbol: SYMBOL,
            style: Style::default().fg(SYMBOL_COLOR),
        }
    }

    pub fn calculate_camera(&self, area: Rect) -> (i32, i32) {
        (
            self.position.0 as i32 - area.width as i32 / 2,
            self.position.1 as i32 - area.height as i32 / 2,
        )
    }

    // handles the deplacement of the Player
    fn handle_move(&mut self, dx: i32, dy: i32, map: &mut Map) {
        let new_x = self.position.0 + dx;
        let new_y = self.position.1 + dy;

        if let Some(tile) = map.get_tile(new_x, new_y) {
            if !tile.solid {
                self.position = (new_x, new_y);
                map.load_around((
                    new_x.div_euclid(CHUNK_SIZE as i32),
                    new_y.div_euclid(CHUNK_SIZE as i32),
                ));
            }
        }
    }
}

impl Drawable for Player {
    fn draw(&self, buffer: &mut Buffer, area: Rect) {
        let position: Position = Position {
            x: area.width / 2,
            y: area.height / 2,
        };
        let player_cell = buffer.cell_mut(position).unwrap();
        player_cell.set_symbol(self.symbol);
        player_cell.set_style(self.style);
    }
}

impl Playable for Player {
    fn process_key(&mut self, key_code: KeyCode, map: &mut Map) {
        match key_code {
            KeyCode::Up => self.handle_move(0, -1, map),
            KeyCode::Down => self.handle_move(0, 1, map),
            KeyCode::Left => self.handle_move(-1, 0, map),
            KeyCode::Right => self.handle_move(1, 0, map),
            _ => {}
        }
    }
}

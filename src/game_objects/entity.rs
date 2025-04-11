use crossterm::event::KeyCode;
use ratatui::{
    buffer::Buffer,
    layout::{Position, Rect},
    style::{Color, Style},
};

use crate::map::map::{CHUNK_SIZE, Map};

pub trait Drawable {
    fn draw(&self, buffer: &mut Buffer, area: Rect);
}

pub enum EntityKind {
    Human,
    Dragon,
    Sheep,
}

impl EntityKind {
    pub fn symbol(&self) -> &'static str {
        match self {
            EntityKind::Human => "@",
            EntityKind::Dragon => "D",
            EntityKind::Sheep => "s",
        }
    }

    pub fn style(&self) -> Style {
        match self {
            EntityKind::Human => Style::default().fg(Color::White),
            EntityKind::Dragon => Style::default().fg(Color::Red),
            EntityKind::Sheep => Style::default().fg(Color::Rgb(255, 209, 223)),
        }
    }
}

// who controls the Entity
#[derive(Copy, Clone)]
pub enum Controller {
    Player,
    // todo: AI
}

impl Controller {
    pub fn update_entity(&self, entity: &mut Entity, input: Option<KeyCode>, map: &mut Map) {
        match self {
            Controller::Player => {
                if let Some(key_code) = input {
                    self.handle_player_input(entity, key_code, map);
                }
            }
        }
    }

    fn handle_player_input(&self, entity: &mut Entity, key_code: KeyCode, map: &mut Map) {
        let (dx, dy) = match key_code {
            KeyCode::Up => (0, -1),
            KeyCode::Down => (0, 1),
            KeyCode::Left => (-1, 0),
            KeyCode::Right => (1, 0),
            _ => (0, 0),
        };

        let new_x = entity.position.0 + dx;
        let new_y = entity.position.1 + dy;

        if let Some(tile) = map.get_tile(new_x, new_y) {
            if !tile.solid {
                entity.position = (new_x, new_y);
                map.load_around((
                    new_x.div_euclid(CHUNK_SIZE as i32),
                    new_y.div_euclid(CHUNK_SIZE as i32),
                ));
            }
        }
    }
}

pub struct Entity {
    kind: EntityKind,
    pub position: (i32, i32),
    pub symbol: &'static str,
    pub style: Style,
    pub controller: Controller,
}

impl Entity {
    pub fn new(kind: EntityKind, position: (i32, i32), controller: Controller) -> Self {
        Self {
            position,
            symbol: kind.symbol(),
            style: kind.style(),
            controller,
            kind,
        }
    }

    pub fn player(position: (i32, i32)) -> Self {
        Self::new(EntityKind::Human, position, Controller::Player)
    }

    pub fn update(&mut self, input: Option<KeyCode>, map: &mut Map) {
        let controller = self.controller;
        controller.update_entity(self, input, map);
    }
}

impl Drawable for Entity {
    fn draw(&self, buffer: &mut Buffer, area: Rect) {
        //let position: Position = Position { x: 0, y: 0 };
        let position: Position = Position {
            x: area.width / 2,
            y: area.height / 2,
        };
        let player_cell = buffer.cell_mut(position).unwrap();
        player_cell.set_symbol(self.symbol);
        player_cell.set_style(self.style);
    }
}

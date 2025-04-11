use crossterm::event::KeyCode;
use ratatui::{buffer::Buffer, layout::Rect};

use crate::{
    game_objects::{GameObject, player::Player},
    map::map::Map,
};

pub struct EntityManager {
    pub player: Player,
    entities: Vec<Box<dyn GameObject>>,
}

impl EntityManager {
    pub fn new() -> Self {
        //let player = Player::new((map.size.0 / 2, map.size.1 / 2));
        let player = Player::new((0, 0));
        Self {
            player,
            entities: Vec::new(),
        }
    }

    pub fn add_entity(&mut self, entity: Box<dyn GameObject>) {
        self.entities.push(entity);
    }

    pub fn process_key(&mut self, key_code: KeyCode, map: &mut Map) {
        match key_code {
            _ => self.player.process_key(key_code, map),
        }
    }

    pub fn draw(&self, buffer: &mut Buffer, area: Rect) {
        for entity in self.entities.iter() {
            entity.draw(buffer, area);
        }
        self.player.draw(buffer, area);
    }
}

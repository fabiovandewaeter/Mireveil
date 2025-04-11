use crossterm::event::KeyCode;
use ratatui::{buffer::Buffer, layout::Rect};

use crate::{
    game_objects::entity::{Drawable, Entity},
    map::map::Map,
};

pub struct EntityManager {
    pub player: Entity,
    entities: Vec<Box<Entity>>,
}

impl EntityManager {
    pub fn new() -> Self {
        let player = Entity::player((0, 0));
        Self {
            player,
            entities: Vec::new(),
        }
    }

    pub fn add_entity(&mut self, entity: Box<Entity>) {
        self.entities.push(entity);
    }

    pub fn update(&mut self, key_code: KeyCode, map: &mut Map) {
        self.player.update(Some(key_code), map);
        for entity in &mut self.entities {
            entity.update(None, map);
        }
    }

    pub fn draw(&self, buffer: &mut Buffer, area: Rect) {
        for entity in self.entities.iter() {
            entity.draw(buffer, area);
        }
        self.player.draw(buffer, area);
    }
}

use crossterm::event::KeyCode;
use ratatui::{buffer::Buffer, layout::Rect};

use crate::{
    game_objects::entity::{Drawable, Entity},
    map::map::Map,
    systems::camera::update_visibility,
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
        update_visibility(self.player.position, 50, map);
        for entity in &mut self.entities {
            entity.update(None, map);
        }
    }

    pub fn draw(&self, buffer: &mut Buffer, area: Rect, camera_position: (i32, i32)) {
        for entity in self.entities.iter() {
            entity.draw(buffer, area, camera_position);
        }
        self.player.draw(buffer, area, camera_position);
    }

    pub fn find_entity_at(&self, world_x: i32, world_y: i32) -> Option<&Entity> {
        // checks if it's the player first
        if self.player.position == (world_x, world_y) {
            return Some(&self.player);
        }
        // else checks if it's another entity
        for entity in self.entities.iter() {
            if entity.position == (world_x, world_y) {
                return Some(entity.as_ref());
            }
        }
        None
    }
}

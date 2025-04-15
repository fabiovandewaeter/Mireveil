use crossterm::event::KeyCode;
use ratatui::{buffer::Buffer, layout::Rect};

use crate::{
    game_objects::entity::{Drawable, Entity},
    map::map::Map,
    systems::camera::update_visibility,
};

pub struct EntityManager {
    pub player: Entity,
    entities: Vec<Entity>,
}

impl EntityManager {
    pub fn new() -> Self {
        let player = Entity::player((0, 0));
        Self {
            player,
            entities: Vec::new(),
        }
    }

    pub fn add_entity(&mut self, entity: Entity) {
        self.entities.push(entity);
    }

    pub fn update(&mut self, key_code: KeyCode, map: &mut Map) {
        self.player
            .update(Some(key_code), map, self.entities.iter_mut());
        update_visibility(self.player.position, 50, map);
        let size = self.entities.len();
        for i in 0..size {
            let (left, right) = self.entities.split_at_mut(i);
            let (current, right) = right.split_first_mut().unwrap();
            let other_entities = left.iter_mut().chain(right.iter_mut());
            current.update(None, map, other_entities);
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
                return Some(entity);
            }
        }
        None
    }
}

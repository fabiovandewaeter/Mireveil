use crossterm::event::KeyCode;
use ratatui::{buffer::Buffer, layout::Rect};

use crate::{common::utils::Drawable, entities::entity::Entity, map::map::Map, menu::Logger};

use super::camera::Camera;

pub struct EntityManager {
    //pub player: Entity,
    entities: Vec<Entity>,
    dead_entities: Vec<Entity>,
}

impl EntityManager {
    pub fn new() -> Self {
        Self {
            entities: vec![Entity::player((0, 0, 0))],
            dead_entities: Vec::new(),
        }
    }

    pub fn add_entity(&mut self, entity: Entity) {
        self.entities.push(entity);
    }

    pub fn update(
        &mut self,
        key_code: KeyCode,
        camera: &Camera,
        map: &mut Map,
        logger: &mut Logger,
    ) {
        let size = self.entities.len();
        for i in 0..size {
            let (left, right) = self.entities.split_at_mut(i);
            let (current, right) = right.split_first_mut().unwrap();
            let mut other_entities: Vec<&mut Entity> =
                left.iter_mut().chain(right.iter_mut()).collect();

            current.update(key_code, map, other_entities.as_mut_slice(), logger);
            if current.is_player() {
                camera.update_visibility(current.position, 50, map);
            }
        }

        self.handle_dead_entities();
    }

    fn handle_dead_entities(&mut self) {
        let size = self.entities.len();
        let mut dead_entity_indices = Vec::new();
        for i in 0..size {
            if self.entities[i].is_dead() {
                dead_entity_indices.push(i);
            }
        }
        for &i in dead_entity_indices.iter().rev() {
            let dead_entity = self.entities.remove(i);
            self.dead_entities.push(dead_entity);
        }
    }

    pub fn find_entity_at(&self, global_coordinates: (i32, i32, i32)) -> Option<&Entity> {
        // checks if it's another entity
        for entity in self.entities.iter() {
            if entity.position == global_coordinates {
                return Some(entity);
            }
        }
        // else checks if it's a dead entity
        for dead_entity in self.dead_entities.iter() {
            if dead_entity.position == global_coordinates {
                return Some(dead_entity);
            }
        }
        None
    }

    pub fn count_living_entities(&self) -> u32 {
        let mut counter = 0;
        for entity in &self.entities {
            if !entity.is_dead() {
                counter += 1;
            }
        }
        counter
    }

    pub fn get_player(&self) -> Option<&Entity> {
        self.entities.iter().find(|e| e.is_player())
    }

    pub fn get_player_position(&self) -> Option<(i32, i32, i32)> {
        if let Some(player) = self.get_player() {
            Some(player.position)
        } else {
            None
        }
    }

    pub fn draw(&self, buffer: &mut Buffer, area: Rect, camera: &Camera, map: &Map) {
        for dead_entity in self.dead_entities.iter() {
            dead_entity.draw(buffer, area, camera, map);
        }
        for entity in self.entities.iter() {
            entity.draw(buffer, area, camera, map);
        }
    }
}

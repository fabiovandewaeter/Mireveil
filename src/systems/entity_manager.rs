use ratatui::{buffer::Buffer, layout::Rect};

use crate::game_objects::GameObject;

pub struct EntityManager {
    entities: Vec<Box<dyn GameObject>>,
}

impl EntityManager {
    pub fn new() -> Self {
        Self {
            entities: Vec::new(),
        }
    }

    pub fn add_entity(&mut self, entity: Box<dyn GameObject>) {
        self.entities.push(entity);
    }

    pub fn draw(&self, buffer: &mut Buffer, area: Rect) {
        for entity in self.entities.iter() {
            entity.draw(buffer, area);
        }
    }
}

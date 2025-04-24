use crate::map::map::Map;

use super::entity::{Entity, EntityKind};

pub trait AiBehavior {
    fn update(&self, entity: &Entity, map: &Map, other_entities: &mut [&mut Entity]);

    fn decide_movement(
        &self,
        entity: &Entity,
        map: &Map,
        other_entities: &mut [&mut Entity],
    ) -> (i32, i32, i32);

    /// clone this behavior into a fresh Box<dyn AiBehavior>
    fn box_clone(&self) -> Box<dyn AiBehavior>;
}

impl Clone for Box<dyn AiBehavior> {
    fn clone(&self) -> Box<dyn AiBehavior> {
        self.box_clone()
    }
}

#[derive(Copy, Clone)]
pub struct ChasePlayerBehavior;

impl AiBehavior for ChasePlayerBehavior {
    fn update(&self, entity: &Entity, map: &Map, other_entities: &mut [&mut Entity]) {
        self.decide_movement(entity, map, other_entities);
    }

    fn decide_movement(
        &self,
        entity: &Entity,
        _map: &Map,
        other_entities: &mut [&mut Entity],
    ) -> (i32, i32, i32) {
        let target = other_entities
            .iter()
            .find(|e| e.kind == EntityKind::Human && !e.is_dead());

        if let Some(target) = target {
            let dx = (target.position.0 - entity.position.0).signum();
            let dy = (target.position.1 - entity.position.1).signum();
            (dx, dy, 0)
        } else {
            (0, 0, 0)
        }
    }

    fn box_clone(&self) -> Box<dyn AiBehavior> {
        Box::new(*self)
    }
}

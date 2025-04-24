use crate::{map::map::Map, menu::Logger};

use super::{
    controller::Controller,
    entity::{Entity, EntityKind, EntityStats},
};

pub trait AIBehavior {
    fn update(
        &self,
        entity_position: (i32, i32, i32),
        entity_stats: &mut EntityStats,
        entity_controller: &mut Controller,
        map: &mut Map,
        other_entities: &mut [&mut Entity],
        logger: &mut Logger,
    );

    fn decide_movement(
        &self,
        entity_position: (i32, i32, i32),
        map: &Map,
        other_entities: &mut [&mut Entity],
    ) -> (i32, i32, i32);

    /// clone this behavior into a fresh Box<dyn AiBehavior>
    fn box_clone(&self) -> Box<dyn AIBehavior>;
}

impl Clone for Box<dyn AIBehavior> {
    fn clone(&self) -> Box<dyn AIBehavior> {
        self.box_clone()
    }
}

#[derive(Copy, Clone)]
pub struct ChasePlayerBehavior;

impl AIBehavior for ChasePlayerBehavior {
    fn update(
        &self,
        entity_position: (i32, i32, i32),
        entity_stats: &mut EntityStats,
        entity_controller: &mut Controller,
        map: &mut Map,
        other_entities: &mut [&mut Entity],
        logger: &mut Logger,
    ) {
        let (new_x, new_y, new_z) = self.decide_movement(entity_position, map, other_entities);
        entity_controller.handle_entity_movement(
            entity,
            new_x,
            new_y,
            new_z,
            map,
            other_entities,
            logger,
        );
    }

    fn decide_movement(
        &self,
        entity_position: (i32, i32, i32),
        _map: &Map,
        other_entities: &mut [&mut Entity],
    ) -> (i32, i32, i32) {
        let target = other_entities
            .iter()
            .find(|e| e.kind == EntityKind::Human && !e.is_dead());

        if let Some(target) = target {
            let dx = (target.position.0 - entity_position.0).signum();
            let dy = (target.position.1 - entity_position.1).signum();
            (dx, dy, 0)
        } else {
            (0, 0, 0)
        }
    }

    fn box_clone(&self) -> Box<dyn AIBehavior> {
        Box::new(*self)
    }
}

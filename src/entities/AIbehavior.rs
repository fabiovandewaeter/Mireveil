use crate::{map::map::Map, menu::Logger};

use super::entity::{Entity, EntityKind};

pub trait AIBehavior {
    fn update(
        &self,
        /*entity_position: (i32, i32, i32),
        entity_stats: &mut EntityStats,
        entity_controller: &mut Controller,*/
        entity: &mut Entity,
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
        /*entity_position: (i32, i32, i32),
        entity_stats: &mut EntityStats,
        entity_controller: &mut Controller,*/
        entity: &mut Entity,
        map: &mut Map,
        other_entities: &mut [&mut Entity],
        logger: &mut Logger,
    ) {
        let (new_x, new_y, new_z) = self.decide_movement(entity.position, map, other_entities);
        let controller = entity.controller.clone();
        let (x, y, z) = entity.position;
        controller.handle_entity_movement(entity, new_x, new_y, new_z, map, other_entities, logger);
        let (bx, by, bz) = entity.position;
        logger.push_message(format!(
            "entity moves {} {} {} | {} {} {}",
            x, y, z, bx, by, bz
        ));
    }

    /// returns new position the entity wants to reach
    fn decide_movement(
        &self,
        entity_position: (i32, i32, i32),
        _map: &Map,
        other_entities: &mut [&mut Entity],
    ) -> (i32, i32, i32) {
        let target = other_entities
            .iter()
            .find(|e| e.kind == EntityKind::Human && !e.is_dead());

        let (mut new_x, mut new_y, mut new_z) = entity_position;
        (new_x + 1, new_y, new_z)
    }

    fn box_clone(&self) -> Box<dyn AIBehavior> {
        Box::new(*self)
    }
}

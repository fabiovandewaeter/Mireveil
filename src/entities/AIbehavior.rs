use crate::{map::map::Map, menu::Logger};

use super::entity::Entity;

pub trait AIBehavior {
    fn update(
        &self,
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
        entity: &mut Entity,
        map: &mut Map,
        other_entities: &mut [&mut Entity],
        logger: &mut Logger,
    ) {
        let (new_x, new_y, new_z) = self.decide_movement(entity.position, map, other_entities);
        let controller = entity.controller.clone();
        controller.handle_entity_movement(entity, new_x, new_y, new_z, map, other_entities, logger);
    }

    /// returns new position the entity wants to reach
    fn decide_movement(
        &self,
        entity_position: (i32, i32, i32),
        _map: &Map,
        _other_entities: &mut [&mut Entity],
    ) -> (i32, i32, i32) {
        // example : try to reach (0, 0, 0)
        let (current_x, current_y, current_z) = entity_position;

        let (target_x, target_y, target_z) = (0, 0, 0);
        let dx = target_x - current_x;
        let dy = target_y - current_y;
        let dz = target_z - current_z;

        let step_x = dx.signum();
        let step_y = dy.signum();
        let step_z = dz.signum();

        (current_x + step_x, current_y + step_y, current_z + step_z)
    }

    fn box_clone(&self) -> Box<dyn AIBehavior> {
        Box::new(*self)
    }
}

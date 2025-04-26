use crossterm::event::KeyCode;

use crate::{
    common::utils::Drawable,
    map::map::{CHUNK_SIZE, Direction, Map},
    menu::Logger,
};

use super::{AIbehavior::AIBehavior, entity::Entity};

/// who controls the Entity
#[derive(Clone)]
pub enum Controller {
    Player,
    AI(Box<dyn AIBehavior>),
}

impl Controller {
    pub fn update_entity(
        &self,
        entity: &mut Entity,
        input: KeyCode,
        map: &mut Map,
        other_entities: &mut [&mut Entity],
        logger: &mut Logger,
    ) {
        match self {
            Controller::Player => {
                self.handle_player_input(entity, input, map, other_entities, logger);
            }
            Controller::AI(behavior) => {
                behavior.update(entity, map, other_entities, logger);
            }
        }
    }

    /// moves the player and changes his direction
    fn handle_player_input(
        &self,
        entity: &mut Entity,
        key_code: KeyCode,
        map: &mut Map,
        other_entities: &mut [&mut Entity],
        logger: &mut Logger,
    ) {
        match key_code {
            KeyCode::Char('e') => {
                let coordinates_tile_entity_looks_at =
                    entity.direction.coordinates_in_front(entity.position);
                logger.push_message(format!(
                    "{} {} {}",
                    coordinates_tile_entity_looks_at.0,
                    coordinates_tile_entity_looks_at.1,
                    coordinates_tile_entity_looks_at.2
                ));
                if let Some(tile_entity_looks_at) =
                    map.get_tile_mut(coordinates_tile_entity_looks_at)
                {
                    if let Some(structure) = tile_entity_looks_at.structure.as_mut() {
                        structure.interact(logger);
                    }
                }
            }
            _ => {}
        }

        // handles movements
        let (dx, dy, dz) = match key_code {
            KeyCode::Up => (0, -1, 0),
            KeyCode::Down => (0, 1, 0),
            KeyCode::Left => (-1, 0, 0),
            KeyCode::Right => (1, 0, 0),
            KeyCode::Char('y') => (0, 0, 1),
            KeyCode::Char('u') => (0, 0, -1),
            _ => (0, 0, 0),
        };

        self.handle_entity_movement(entity, dx, dy, dz, map, other_entities, logger);
    }

    /// moves the entity, changes its direction and attack the entity at the new position by adding the delta in the 3 directions, and handle xp gain and load map around new position
    pub fn handle_entity_movement(
        &self,
        entity: &mut Entity,
        dx: i32,
        dy: i32,
        dz: i32,
        map: &mut Map,
        other_entities: &mut [&mut Entity],
        logger: &mut Logger,
    ) {
        // changes direction of the entity
        if dy < 0 {
            entity.direction = Direction::North;
        } else if dy > 0 {
            entity.direction = Direction::South;
        }
        if dx < 0 {
            entity.direction = Direction::West;
        } else if dx > 0 {
            entity.direction = Direction::East;
        }

        let new_x = entity.position.0 + dx;
        let new_y = entity.position.1 + dy;
        let new_z = entity.position.2 + dz;

        // if an entity is on new position, attacks it, else move to new position
        if let Some(target) = other_entities
            .iter_mut()
            .find(|e| e.position == (new_x, new_y, new_z))
        {
            let target_was_alive = !target.is_dead();

            // attacks the entity if collision
            let target_coordinates = (new_x, new_y, new_z);
            for action in &entity.actions {
                if action.handle_mana_cost(&mut entity.stats) {
                    action.affect(entity, target_coordinates, other_entities, logger);
                }
            }

            // if collided with a living target
            if target_was_alive {
                if let Some(target) = other_entities
                    .iter_mut()
                    .find(|e| e.position == (new_x, new_y, new_z))
                {
                    // if the target is now dead
                    if target_was_alive && target.is_dead() {
                        logger.push_message(format!(
                            "{} xp needed for next level",
                            entity.level_manager.xp_to_next_level()
                        ));
                        Self::handle_xp_gain(entity, target, logger);
                    }
                }
            }
        } else {
            map.load_around((
                new_x.div_euclid(CHUNK_SIZE as i32),
                new_y.div_euclid(CHUNK_SIZE as i32),
                new_z,
            ));
            if let Some(tile) = map.get_tile((new_x, new_y, new_z)) {
                if tile.walkable() {
                    entity.position = (new_x, new_y, new_z);
                }
            }
        }
    }

    fn handle_xp_gain(attacker: &mut Entity, target: &mut Entity, logger: &mut Logger) {
        let xp_gained = target.xp_drop;

        let levels_gained = attacker
            .level_manager
            .add_xp(xp_gained, &mut attacker.stats);
        logger.push_message(format!("{} gained {} XP", attacker.symbol(), xp_gained));

        if levels_gained > 0 {
            logger.push_message(format!(
                "{} reached level {}",
                attacker.symbol(),
                attacker.level_manager.level
            ));
        }
    }
}

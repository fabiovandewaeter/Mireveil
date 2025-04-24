use crossterm::event::KeyCode;

use crate::{
    map::map::{CHUNK_SIZE, Map},
    menu::Logger,
};

use super::{AIbehavior::AiBehavior, entity::Entity};

/// who controls the Entity
#[derive(Clone)]
pub enum Controller {
    Player,
    AI(Box<dyn AiBehavior>),
}

impl Controller {
    pub fn update_entity(
        &self,
        entity: &mut Entity,
        input: Option<KeyCode>,
        map: &mut Map,
        other_entities: &mut [&mut Entity],
        logger: &mut Logger,
    ) {
        match self {
            Controller::Player => {
                if let Some(key_code) = input {
                    self.handle_player_input(entity, key_code, map, other_entities, logger);
                }
            }
            Controller::AI(behavior) => {
                behavior.update(entity, map, other_entities);
            }
        }
    }

    fn handle_player_input(
        &self,
        entity: &mut Entity,
        key_code: KeyCode,
        map: &mut Map,
        other_entities: &mut [&mut Entity],
        logger: &mut Logger,
    ) {
        let (dx, dy, dz) = match key_code {
            KeyCode::Up => (0, -1, 0),
            KeyCode::Down => (0, 1, 0),
            KeyCode::Left => (-1, 0, 0),
            KeyCode::Right => (1, 0, 0),
            KeyCode::Char('y') => (0, 0, 1),
            KeyCode::Char('u') => (0, 0, -1),
            _ => (0, 0, 0),
        };

        let new_x = entity.position.0 + dx;
        let new_y = entity.position.1 + dy;
        let new_z = entity.position.2 + dz;

        self.handle_entity_movement(entity, new_x, new_y, new_z, map, other_entities, logger);
    }

    pub fn handle_entity_movement(
        &self,
        entity: &mut Entity,
        new_x: i32,
        new_y: i32,
        new_z: i32,
        map: &mut Map,
        other_entities: &mut [&mut Entity],
        logger: &mut Logger,
    ) {
        let mut target_was_alive = false;
        if let Some(target) = other_entities
            .iter_mut()
            .find(|e| e.position == (new_x, new_y, new_z))
        {
            target_was_alive = !target.is_dead();

            let target_coordinates = (new_x, new_y, new_z);
            // attacks the entity if collision
            for action in &entity.actions {
                if action.handle_mana_cost(&mut entity.stats) {
                    action.affect(entity, target_coordinates, other_entities, logger);
                }
            }
        }

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

        map.load_around((
            new_x.div_euclid(CHUNK_SIZE as i32),
            new_y.div_euclid(CHUNK_SIZE as i32),
            new_z,
        ));
        if let Some(tile) = map.get_tile((new_x, new_y, new_z)) {
            if !tile.solid {
                entity.position = (new_x, new_y, new_z);
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

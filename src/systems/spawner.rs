use std::time::{Duration, Instant};

use rand::{
    Rng,
    distr::{Distribution, weighted::WeightedIndex},
    rng,
};

use crate::{
    entities::entity::{Controller, Entity, EntityKind},
    map::map::Map,
};

use super::entity_manager::EntityManager;

pub struct SpawnerConfiguration {
    interval: Duration,
    // spawn entities only if total quantity of entities is lower to max_entities
    max_entities: usize,
    spawn_radius: i32,
    spawn_around_player: bool,
    spawn_chances: Vec<(EntityKind, f32)>,
}

impl Default for SpawnerConfiguration {
    fn default() -> Self {
        Self {
            interval: Duration::from_secs(3),
            max_entities: 10,
            spawn_radius: 20,
            spawn_around_player: true,
            spawn_chances: vec![
                (EntityKind::Human, 0.1),
                (EntityKind::Sheep, 0.5),
                (EntityKind::Dragon, 0.001),
            ],
        }
    }
}

pub struct Spawner {
    pub config: SpawnerConfiguration,
    pub last_spawn: Instant,
}

impl Spawner {
    pub fn new(config: SpawnerConfiguration) -> Self {
        Self {
            config,
            last_spawn: Instant::now(),
        }
    }

    // spawns an entity if cooldown has passed and if there is an empty tile
    pub fn try_spawn(&mut self, entity_manager: &mut EntityManager, map: &Map) {
        let now = Instant::now();
        if now.duration_since(self.last_spawn) < self.config.interval {
            return;
        }

        let mut rng = rng();
        let kinds: Vec<EntityKind> = self
            .config
            .spawn_chances
            .iter()
            .map(|(kind, _)| *kind)
            .collect();
        let weights: Vec<f32> = self
            .config
            .spawn_chances
            .iter()
            .map(|(_, weight)| *weight)
            .collect();
        let dist = WeightedIndex::new(&weights)
            .expect("Error creating WeightedIndex in Spawner::try_spawn()");
        let chosen_kind = kinds[dist.sample(&mut rng)];

        // spawn around player or world spawn (0,0)
        let (base_x, base_y) = if self.config.spawn_around_player {
            entity_manager.player.position
        } else {
            (0, 0)
        };

        let offset_x = rng.random_range(-self.config.spawn_radius..=self.config.spawn_radius);
        let offset_y = rng.random_range(-self.config.spawn_radius..=self.config.spawn_radius);
        let spawn_x = base_x + offset_x;
        let spawn_y = base_y + offset_y;

        if entity_manager.find_entity_at(spawn_x, spawn_y).is_none()
            && !map.get_tile(spawn_x, spawn_y).unwrap().solid
        {
            self.last_spawn = now;
            let new_entity = Entity::new(chosen_kind, (spawn_x, spawn_y), Controller::AI);
            // only spawns the entity if there is not too many entities on the map
            if entity_manager.count_living_entities() < self.config.max_entities as u32 {
                entity_manager.add_entity(new_entity);
            }
        }
    }
}

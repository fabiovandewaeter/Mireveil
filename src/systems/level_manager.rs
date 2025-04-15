use crate::entities::entity::{Entity, EntityStats};

trait XPCurve {
    // returns the xp needed to reach that level
    fn xp_required(&self, level: u32) -> u32;
}

struct ExponentialCurve {
    // base xp required
    base: u32,
    exponent: f32,
}

impl ExponentialCurve {
    fn new(base: u32, exponent: f32) -> Self {
        Self { base, exponent }
    }
}

impl Default for ExponentialCurve {
    fn default() -> Self {
        Self {
            base: 2,
            exponent: 1.5,
        }
    }
}

impl XPCurve for ExponentialCurve {
    fn xp_required(&self, level: u32) -> u32 {
        // XP required = base * level^exponent, rounded up
        (self.base as f32 * (level as f32).powf(self.exponent)).ceil() as u32
    }
}

pub struct LevelManager {
    pub level: u32,
    current_xp: u32,
    xp_curve: Box<dyn XPCurve>,
}

impl LevelManager {
    pub fn new(initial_level: u32, xp_curve: Box<dyn XPCurve>) -> Self {
        Self {
            level: initial_level,
            current_xp: 0,
            xp_curve,
        }
    }

    pub fn add_xp(&mut self, xp: u32, entity_stats: &mut EntityStats) -> u32 {
        let initial_level = self.level;

        self.current_xp += xp;
        // checks if there are any level ups
        while (self.current_xp >= self.xp_curve.xp_required(self.level + 1)) {
            self.handle_level_up(entity_stats);
        }
        self.level - initial_level
    }

    // returns the required xp for next lvl
    pub fn xp_to_next_level(&self) -> u32 {
        self.xp_curve.xp_required(self.level + 1)
    }

    // add stats and increase level of the entity
    fn handle_level_up(&mut self, entity_stats: &mut EntityStats) {
        self.current_xp -= self.xp_curve.xp_required(self.level + 1);
        self.level += 1;

        // increase max_hp and reset current hp
        let bonus_hp = (entity_stats.max_hp as f32 * 0.1).ceil() as u32;
        entity_stats.max_hp += bonus_hp;
        entity_stats.hp = entity_stats.max_hp;

        // increase max_mana and reset current mana
        let bonus_mana = (entity_stats.max_mana as f32 * 0.1).ceil() as u32;
        entity_stats.max_mana += bonus_mana;
        entity_stats.mana = entity_stats.max_mana;

        entity_stats.defense += 1;
        entity_stats.strength += 1;
        entity_stats.magic += 1;
    }
}

impl Default for LevelManager {
    fn default() -> Self {
        Self {
            level: 1,
            current_xp: 0,
            xp_curve: Box::new(ExponentialCurve::default()),
        }
    }
}

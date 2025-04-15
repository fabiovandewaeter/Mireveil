use super::entity::Entity;

pub trait Action {
    fn affect(&self, source: &Entity, target: &mut Entity) -> Option<String>;
}

pub struct Attack {
    damage: u32,
    // TODO: element, range etc.
}

impl Attack {
    pub fn new(damage: u32) -> Self {
        Self { damage }
    }
}

impl Action for Attack {
    fn affect(&self, source: &Entity, target: &mut Entity) -> Option<String> {
        let actual_damage = target.take_damage(self.damage + source.stats.strength);
        Some(format!(
            "{} attacks {} (-{} PV){}",
            source.symbol(),
            target.symbol(),
            actual_damage,
            if target.is_dead() { " and dies!" } else { "" }
        ))
    }
}

use crate::{entities::entity::Entity, menu::Logger};

pub trait Action {
    fn affect(&self, source: &Entity, target: &mut Entity, logger: &mut Logger);
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
    fn affect(&self, source: &Entity, target: &mut Entity, logger: &mut Logger) {
        let target_was_alive = !target.is_dead();
        let mut damage = self.damage + source.stats.strength;
        if let Some(weapon_datas) = source.get_weapon_datas() {
            damage += weapon_datas.strenght;
        }
        let actual_damage = target.take_damage(damage);
        logger.push_message(format!(
            "{} attacks {} (-{} PV){}",
            source.symbol(),
            target.symbol(),
            actual_damage,
            if target_was_alive && target.is_dead() {
                " and it died"
            } else {
                ""
            }
        ));
    }
}

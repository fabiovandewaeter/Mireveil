use std::collections::HashSet;

use crate::{
    entities::entity::{Entity, EntityStats},
    map::map::Map,
    menu::Logger,
};

#[derive(Clone, Copy, PartialEq)]
pub enum ActionType {
    Physical,
    Fire,
    Ice,
    Lightning,
    Healing,
}

pub trait Action {
    fn affect(
        &self,
        source: &Entity,
        target_coordinates: (i32, i32, i32),
        other_entities: &mut [&mut Entity],
        logger: &mut Logger,
    );

    /// returns true if the entity has enough mana to do the Action, and reduce its mana; otherwise returns false
    fn handle_mana_cost(&self, source_stats: &mut EntityStats) -> bool {
        let mana_cost = self.mana_cost();
        if source_stats.mana >= mana_cost {
            source_stats.mana -= mana_cost;
            return true;
        }
        false
    }

    /// 1 by default
    fn range(&self) -> u32 {
        1
    }

    fn attack_type(&self) -> ActionType;

    fn name(&self) -> &str;

    /// 0 by default
    fn mana_cost(&self) -> u32 {
        0
    }

    /// only the target_pos by default
    fn get_area_of_effect(
        &self,
        source_pos: (i32, i32, i32),
        target_pos: (i32, i32, i32),
        map: &Map,
    ) -> HashSet<(i32, i32, i32)> {
        let mut targets = HashSet::new();
        targets.insert(target_pos);
        targets
    }
}

pub struct MeleeAttack {
    name: String,
    damage: u32,
    attack_type: ActionType,
}

impl MeleeAttack {
    pub fn new(name: &str, damage: u32, attack_type: ActionType) -> Self {
        Self {
            name: name.to_string(),
            damage,
            attack_type,
        }
    }

    pub fn calculate_damage(&self, source: &Entity) -> u32 {
        let mut damage = self.damage + source.stats.strength;
        if let Some(weapon_data) = source.get_weapon_data() {
            damage += weapon_data.strenght;
        }
        damage
    }
}

impl Action for MeleeAttack {
    fn affect(
        &self,
        source: &Entity,
        target_coordinates: (i32, i32, i32),
        other_entities: &mut [&mut Entity],
        logger: &mut Logger,
    ) {
        if let Some(target) = other_entities
            .iter_mut()
            .find(|e| e.position == target_coordinates)
        {
            let target_was_alive = !target.is_dead();
            let damage = self.calculate_damage(source);
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

    fn name(&self) -> &str {
        &self.name
    }

    fn attack_type(&self) -> ActionType {
        self.attack_type
    }
}

pub struct AreaAttack {
    pub name: String,
    pub damage: u32,
    pub attack_range: u32,
    pub area_radius: u32,
    pub attack_type: ActionType,
    pub mana_cost: u32,
}

impl AreaAttack {
    pub fn new(
        name: &str,
        damage: u32,
        range: u32,
        radius: u32,
        attack_type: ActionType,
        mana_cost: u32,
    ) -> Self {
        Self {
            damage,
            name: name.to_string(),
            attack_range: range,
            area_radius: radius,
            attack_type,
            mana_cost,
        }
    }

    pub fn calculate_damage(&self, source: &Entity) -> u32 {
        let mut damage = self.damage + source.stats.strength;
        if let Some(weapon_data) = source.get_weapon_data() {
            damage += weapon_data.strenght;
        }
        damage
    }
}

impl Action for AreaAttack {
    fn affect(
        &self,
        source: &Entity,
        target_coordinates: (i32, i32, i32),
        other_entities: &mut [&mut Entity],
        logger: &mut Logger,
    ) {
        let area = self.get_area_of_effect(source.position, target_coordinates, &Map::default());
        let mut killed_count = 0;
        let mut affected_count = 0;
        let damage = self.calculate_damage(source);

        // attacks all entities in the area
        for target in other_entities.iter_mut() {
            if area.contains(&target.position) {
                let target_was_alive = !target.is_dead();
                let actual_damage = target.take_damage(damage);

                affected_count += 1;
                if target_was_alive && target.is_dead() {
                    killed_count += 1;
                }

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

        if affected_count > 0 {
            logger.push_message(format!(
                "{} attacks {} and hits {} entity{} (-{} PV){}",
                source.symbol(),
                self.name,
                affected_count,
                if affected_count > 1 { "s" } else { "" },
                damage,
                if killed_count > 0 {
                    format!(
                        ", {} entity{} dead{}",
                        killed_count,
                        if killed_count > 1 { "s" } else { "" },
                        if killed_count > 1 { "s" } else { "" }
                    )
                } else {
                    String::new()
                }
            ));
        } else {
            logger.push_message(format!(
                "{} attacks {} but does not reach any entities",
                source.symbol(),
                self.name
            ));
        }
    }

    fn range(&self) -> u32 {
        self.attack_range
    }

    fn name(&self) -> &str {
        &self.name
    }

    fn attack_type(&self) -> ActionType {
        self.attack_type
    }

    fn mana_cost(&self) -> u32 {
        self.mana_cost
    }

    fn get_area_of_effect(
        &self,
        _source_pos: (i32, i32, i32),
        target_pos: (i32, i32, i32),
        _map: &Map,
    ) -> HashSet<(i32, i32, i32)> {
        let mut targets = HashSet::new();
        let (tx, ty, tz) = target_pos;

        for y in (ty - self.area_radius as i32)..=(ty + self.area_radius as i32) {
            for x in (tx - self.area_radius as i32)..=(tx + self.area_radius as i32) {
                let dx = x - tx;
                let dy = y - ty;
                if (dx * dx + dy * dy) <= (self.area_radius * self.area_radius) as i32 {
                    targets.insert((x, y, tz));
                }
            }
        }

        targets
    }
}

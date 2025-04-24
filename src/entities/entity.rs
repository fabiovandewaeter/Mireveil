use std::collections::HashMap;

use crossterm::event::KeyCode;
use ratatui::{
    buffer::Buffer,
    layout::{Position, Rect},
    style::{Color, Style},
};

use crate::{
    actions::action::{Action, ActionType, AreaAttack, MeleeAttack},
    common::utils::Drawable,
    items::item::{EquipmentSlot, Item, ItemKind, WeaponData},
    map::map::{CHUNK_SIZE, Map},
    menu::Logger,
    systems::{camera::Camera, level_manager::LevelManager},
};

#[derive(Clone, Copy)]
pub enum EntityKind {
    Human,
    Dragon,
    Sheep,
}

impl EntityKind {
    pub fn name(&self) -> &'static str {
        match self {
            EntityKind::Human => "Human",
            EntityKind::Dragon => "Dragon",
            EntityKind::Sheep => "Sheep",
        }
    }
    fn symbol(&self) -> &'static str {
        match self {
            EntityKind::Human => "@",
            EntityKind::Dragon => "D",
            EntityKind::Sheep => "S",
        }
    }

    fn style(&self) -> Style {
        match self {
            EntityKind::Human => Style::default().fg(Color::Rgb(255, 255, 255)),
            EntityKind::Dragon => Style::default().fg(Color::Rgb(255, 0, 0)),
            EntityKind::Sheep => Style::default().fg(Color::Rgb(255, 209, 223)),
        }
    }

    fn stats(&self) -> EntityStats {
        match self {
            EntityKind::Human => EntityStats {
                max_hp: 100,
                hp: 100,
                max_mana: 100,
                mana: 100,
                defense: 5,
                strength: 5,
                magic: 5,
            },
            EntityKind::Dragon => EntityStats {
                max_hp: 1000,
                hp: 1000,
                max_mana: 200,
                mana: 200,
                defense: 50,
                strength: 50,
                magic: 20,
            },
            EntityKind::Sheep => EntityStats {
                max_hp: 30,
                hp: 30,
                max_mana: 0,
                mana: 0,
                defense: 5,
                strength: 1,
                magic: 0,
            },
        }
    }

    fn actions(&self) -> Vec<Box<dyn Action>> {
        match self {
            EntityKind::Human => {
                vec![
                    Box::new(MeleeAttack::new("melee human", 10, ActionType::Physical)),
                    Box::new(AreaAttack::new(
                        "area human",
                        10,
                        100,
                        100,
                        ActionType::Physical,
                        0,
                    )),
                ]
            }
            EntityKind::Dragon => {
                vec![Box::new(MeleeAttack::new(
                    "melee dragon",
                    10,
                    ActionType::Fire,
                ))]
            }
            EntityKind::Sheep => {
                vec![Box::new(MeleeAttack::new(
                    "melee sheep",
                    10,
                    ActionType::Physical,
                ))]
            }
        }
    }
}

/// who controls the Entity
#[derive(Copy, Clone)]
pub enum Controller {
    Player,
    AI,
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
            Controller::AI => {}
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

    fn handle_entity_movement(
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
        // TODO: remove that line
        //entity.position.2 = new_z;
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

#[derive(Default)]
pub struct EntityStats {
    pub max_hp: u32,
    pub hp: u32,
    pub max_mana: u32,
    pub mana: u32,
    pub defense: u32,
    pub strength: u32,
    pub magic: u32,
}

struct Inventory {
    items: Vec<Item>,
}

impl Inventory {
    fn new() -> Self {
        Self { items: Vec::new() }
    }

    fn add(&mut self, item: Item) {
        self.items.push(item);
    }
}

pub struct Entity {
    kind: EntityKind,
    pub name: String,
    pub position: (i32, i32, i32),
    controller: Controller,
    pub stats: EntityStats,
    xp_drop: u32,
    level_manager: LevelManager,
    actions: Vec<Box<dyn Action>>,
    equipment: HashMap<EquipmentSlot, Item>,
    inventory: Inventory,
}

impl Entity {
    pub fn new(
        kind: EntityKind,
        name: String,
        position: (i32, i32, i32),
        controller: Controller,
    ) -> Self {
        Self {
            name,
            position,
            controller,
            stats: kind.stats(),
            xp_drop: 10,
            level_manager: LevelManager::default(),
            actions: kind.actions(),
            kind,
            equipment: HashMap::new(),
            inventory: Inventory::new(),
        }
    }

    pub fn symbol(&self) -> &'static str {
        self.kind.symbol()
    }

    pub fn style(&self) -> Style {
        self.kind.style()
    }

    pub fn player(position: (i32, i32, i32)) -> Self {
        let god_sword = Item::new_weapon(
            "GodSword".to_string(),
            "GodSword to test items".to_string(),
            crate::items::item::WeaponKind::Sword,
            1000,
        );
        let mut player = Self::new(
            EntityKind::Human,
            "Player".to_string(),
            position,
            Controller::Player,
        );
        player.equip_item(god_sword);
        player
    }

    pub fn update(
        &mut self,
        input: Option<KeyCode>,
        map: &mut Map,
        other_entities: &mut [&mut Entity],
        logger: &mut Logger,
    ) {
        let controller = self.controller;
        controller.update_entity(self, input, map, other_entities, logger);
    }

    pub fn take_damage(&mut self, amount: u32) -> u32 {
        let damage = std::cmp::min(amount, self.stats.hp);
        self.stats.hp -= damage;
        damage
    }

    pub fn is_dead(&self) -> bool {
        self.stats.hp == 0
    }

    fn equip_item(&mut self, item: Item) {
        if let Some(slot) = item.get_equipment_slot() {
            if let Some(prev) = self.equipment.remove(&slot) {
                self.inventory.add(prev);
            }

            self.equipment.insert(slot, item);
        }
    }

    pub fn get_weapon_data(&self) -> Option<WeaponData> {
        if let Some(weapon) = self.equipment.get(&EquipmentSlot::Hand) {
            match &weapon.kind {
                ItemKind::Weapon(weapon_data) => Some(*weapon_data),
                _ => None,
            }
        } else {
            None
        }
    }
}

impl Drawable for Entity {
    fn draw(&self, buffer: &mut Buffer, area: Rect, camera: &Camera, map: &Map) {
        let on_visible_layer = self.position.2 == camera.position.2;
        let on_visible_tile = camera.is_visible_tile(self.position, map);
        // only draws if the Entity is close enough to the camera and on the visible layer
        if camera.is_point_on_screen(self.position, area) && on_visible_layer && on_visible_tile {
            let screen_x = self.position.0 - camera.position.0;
            let screen_y = self.position.1 - camera.position.1;

            let position: Position = Position {
                x: screen_x as u16,
                y: screen_y as u16,
            };

            let mut style = self.style();
            let mut symbol = self.symbol();

            // changes the style and symbol if the entity is dead
            if self.is_dead() {
                style = style.fg(Camera::style_to_greyscale(style.fg.unwrap_or(Color::Gray)));
                symbol = "†";
            }

            if let Some(cell) = buffer.cell_mut(position) {
                cell.set_symbol(symbol);
                cell.set_style(style);
            }
        }
    }
}

use std::collections::HashMap;

use crossterm::event::KeyCode;
use ratatui::{
    buffer::Buffer,
    layout::{Position, Rect},
    style::{Color, Style},
};

use crate::{
    actions::action::{Action, ActionType, MeleeAttack},
    common::{inventory::Inventory, utils::Drawable},
    items::item::{EquipmentSlot, Item, ItemKind, WeaponData},
    map::map::Map,
    menu::Logger,
    systems::{camera::Camera, level_manager::LevelManager},
};

use super::controller::Controller;

#[derive(Clone, Copy, PartialEq)]
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
                    /*Box::new(AreaAttack::new(
                        "area human",
                        10,
                        100,
                        100,
                        ActionType::Physical,
                        0,
                    )),*/
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

pub struct Entity {
    pub kind: EntityKind,
    pub name: String,
    pub position: (i32, i32, i32),
    pub controller: Controller,
    pub stats: EntityStats,
    pub xp_drop: u32,
    pub level_manager: LevelManager,
    pub actions: Vec<Box<dyn Action>>,
    pub equipment: HashMap<EquipmentSlot, Item>,
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

    pub fn is_player(&self) -> bool {
        match &self.controller {
            Controller::Player => true,
            _ => false,
        }
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
        input: KeyCode,
        map: &mut Map,
        other_entities: &mut [&mut Entity],
        logger: &mut Logger,
    ) {
        let controller = self.controller.clone();
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
        let mut style = self.style();
        let mut symbol = self.symbol();

        // changes the style and symbol if the entity is dead
        if self.is_dead() {
            style = style.fg(Camera::style_to_greyscale(style.fg.unwrap_or(Color::Gray)));
            symbol = "†";
        }
        camera.draw_from_global_coordinates(symbol, style, self.position, buffer, area, map);
    }
}

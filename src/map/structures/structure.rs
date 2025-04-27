use ratatui::style::Color;

use crate::{
    common::{inventory::Inventory, utils::Drawable},
    entities::entity::Entity,
    map::map::Map,
    menu::Logger,
};

pub trait Structure: Drawable {
    /// true by default
    fn block_sight(&self) -> bool {
        true
    }

    /// false by default
    fn walkable(&self) -> bool {
        false
    }

    fn interact(
        &mut self,
        entity: &mut Entity,
        map: &mut Map,
        other_entities: &mut [&mut Entity],
        logger: &mut Logger,
    );
}

pub struct Chest {
    inventory: Inventory,
    is_open: bool,
}

impl Chest {
    pub fn new() -> Self {
        Self {
            inventory: Inventory::new(),
            is_open: false,
        }
    }
}

impl Structure for Chest {
    fn block_sight(&self) -> bool {
        false
    }

    fn interact(
        &mut self,
        entity: &mut Entity,
        map: &mut Map,
        other_entities: &mut [&mut Entity],
        logger: &mut Logger,
    ) {
        logger.push_message(format!("open the chest"));
    }
}

impl Drawable for Chest {
    fn symbol(&self) -> &'static str {
        "c"
    }

    fn color(&self) -> Color {
        Color::Rgb(95, 65, 33)
    }
}

pub struct Wall {}

impl Structure for Wall {
    fn interact(
        &mut self,
        entity: &mut Entity,
        map: &mut Map,
        other_entities: &mut [&mut Entity],
        logger: &mut Logger,
    ) {
    }
}

impl Drawable for Wall {
    fn symbol(&self) -> &'static str {
        "#"
    }

    fn color(&self) -> Color {
        Color::Rgb(150, 150, 150)
    }
}

pub struct Door {
    is_open: bool,
}

impl Door {
    pub fn new() -> Self {
        Self { is_open: false }
    }
}

impl Structure for Door {
    fn block_sight(&self) -> bool {
        !self.is_open
    }

    fn walkable(&self) -> bool {
        self.is_open
    }

    fn interact(
        &mut self,
        entity: &mut Entity,
        map: &mut Map,
        other_entities: &mut [&mut Entity],
        logger: &mut Logger,
    ) {
        if self.is_open {
            logger.push_message(format!("close door"));
        } else {
            logger.push_message(format!("open door"));
        }
        self.is_open = !self.is_open;
    }
}

impl Drawable for Door {
    fn symbol(&self) -> &'static str {
        if self.is_open {
            return "=";
        }
        "|"
    }

    fn color(&self) -> Color {
        Color::Rgb(95, 65, 33)
    }
}

pub struct Stairs {
    goes_up: bool,
}

impl Stairs {
    pub fn new(goes_up: bool) -> Self {
        Self { goes_up }
    }
}

impl Structure for Stairs {
    fn block_sight(&self) -> bool {
        false
    }

    fn walkable(&self) -> bool {
        true
    }

    fn interact(
        &mut self,
        entity: &mut Entity,
        map: &mut Map,
        other_entities: &mut [&mut Entity],
        logger: &mut Logger,
    ) {
        let dz = if self.goes_up { 1 } else { -1 };
        let controller = entity.controller.clone();
        controller.handle_entity_movement(entity, 0, 0, dz, map, other_entities, logger);
    }
}

impl Drawable for Stairs {
    fn symbol(&self) -> &'static str {
        if self.goes_up { "<" } else { ">" }
    }

    fn color(&self) -> Color {
        Color::Rgb(95, 65, 33)
    }
}

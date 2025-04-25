use ratatui::style::Color;

use crate::{
    common::{inventory::Inventory, utils::Drawable},
    menu::Logger,
};

pub trait Structure: Drawable {
    fn block_sight(&self) -> bool; // Bloque le mouvement?
    fn interact(&mut self, logger: &mut Logger); // Retourne un message si interaction
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

    fn interact(&mut self, logger: &mut Logger) {
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
    fn block_sight(&self) -> bool {
        true
    }

    fn interact(&mut self, logger: &mut Logger) {}
}

impl Drawable for Wall {
    fn symbol(&self) -> &'static str {
        "#"
    }

    fn color(&self) -> Color {
        Color::Rgb(150, 150, 150)
    }
}

use std::fmt::format;

use ratatui::style::{Color, Style};

use crate::{common::inventory::Inventory, menu::Logger};

pub trait Structure {
    fn symbol(&self) -> &'static str;
    fn color(&self) -> Color;
    fn style(&self) -> Style {
        Style::default().fg(self.color())
    }
    fn block_sight(&self) -> bool; // Bloque le mouvement?
    fn interact(&mut self, logger: &mut Logger); // Retourne un message si interaction
}

pub struct Chest {
    inventory: Inventory,
    is_open: bool,
}

impl Chest {
    fn new() -> Self {
        Self {
            inventory: Inventory::new(),
            is_open: false,
        }
    }
}

impl Structure for Chest {
    fn symbol(&self) -> &'static str {
        "c"
    }

    fn color(&self) -> Color {
        Color::Rgb(95, 65, 33)
    }

    fn block_sight(&self) -> bool {
        true
    }

    fn interact(&mut self, logger: &mut Logger) {
        logger.push_message(format!("open the chest"));
    }
}

pub struct Wall {}

impl Structure for Wall {
    fn symbol(&self) -> &'static str {
        "#"
    }

    fn color(&self) -> Color {
        Color::Rgb(150, 150, 150)
    }

    fn block_sight(&self) -> bool {
        true
    }

    fn interact(&mut self, logger: &mut Logger) {
        todo!()
    }
}

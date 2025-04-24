use crate::common::{inventory::Inventory, utils::Drawable};

pub struct Chest {
    inventory: Inventory,
}

impl Chest {
    fn new() -> Self {
        Self {
            inventory: Inventory::new(),
        }
    }
}

impl Drawable for Chest {
    fn draw(
        &self,
        buffer: &mut ratatui::prelude::Buffer,
        area: ratatui::prelude::Rect,
        camera: &crate::systems::camera::Camera,
        map: &crate::map::map::Map,
    ) {
        todo!()
    }
}

use ratatui::style::{Color, Style};

use crate::{common::utils::Drawable, entities::entity::Entity, menu::Logger};

use super::{map::Map, structures::structure::Structure};

#[derive(Clone)]
pub enum TileKind {
    Grass,
    Water,
}

impl TileKind {
    pub fn symbol(&self) -> &'static str {
        match self {
            TileKind::Grass => ",",
            TileKind::Water => "~",
        }
    }

    pub fn color(&self) -> Color {
        match self {
            TileKind::Grass => Color::Rgb(0, 102, 0),
            TileKind::Water => Color::Rgb(51, 102, 204),
        }
    }

    pub fn style(&self) -> Style {
        Style::default().fg(self.color())
    }

    pub fn is_walkable(&self) -> bool {
        true
    }
}

pub struct Tile {
    pub kind: TileKind,
    /// the structure on the tile
    pub structure: Option<Box<dyn Structure>>,
}

impl Tile {
    pub fn new(kind: TileKind) -> Self {
        Self {
            structure: None,
            kind,
        }
    }

    pub fn walkable(&self) -> bool {
        // not walkable if there is a structure on the tile
        if let Some(structure) = &self.structure {
            return structure.walkable();
        }
        self.kind.is_walkable()
    }

    /// interacts with the structure on the tile
    fn interact(
        &mut self,
        entity: &mut Entity,
        map: &mut Map,
        other_entities: &mut [&mut Entity],
        logger: &mut Logger,
    ) {
        if let Some(structure) = &mut self.structure {
            structure.interact(entity, map, other_entities, logger);
        }
    }

    pub fn add_structure(&mut self, structure: Box<dyn Structure>) {
        self.structure = Some(structure);
    }

    pub fn block_sight(&self) -> bool {
        if let Some(structure) = &self.structure {
            return structure.block_sight();
        }
        false
    }

    pub fn draw(
        &self,
        buffer: &mut ratatui::prelude::Buffer,
        area: ratatui::prelude::Rect,
        camera: &crate::systems::camera::Camera,
        map: &super::map::Map,
    ) {
        if let Some(structure) = &self.structure {
            // Draw the structure if present
            camera.draw_from_screen_coordinates(
                structure.symbol(),
                structure.style(),
                area.into(),
                buffer,
            );
        } else {
            // Draw the tile itself if no structure
            camera.draw_from_screen_coordinates(&self.symbol(), self.style(), area.into(), buffer);
        }
    }
}

impl Drawable for Tile {
    fn symbol(&self) -> &'static str {
        if let Some(structure) = &self.structure {
            structure.symbol()
        } else {
            self.kind.symbol()
        }
    }

    fn color(&self) -> Color {
        if let Some(structure) = &self.structure {
            structure.color()
        } else {
            self.kind.style().fg.unwrap_or(Color::Reset)
        }
    }

    fn style(&self) -> Style {
        if let Some(structure) = &self.structure {
            structure.style()
        } else {
            self.kind.style()
        }
    }
}

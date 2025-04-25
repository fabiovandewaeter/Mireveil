use ratatui::style::{Color, Style};

use crate::{common::utils::Drawable, menu::Logger};

use super::structures::structure::Structure;

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
    pub symbol: &'static str,
    pub style: Style,
}

impl Tile {
    pub fn new(kind: TileKind) -> Self {
        let symbol = kind.symbol();
        let style = kind.style();
        Self {
            structure: None,
            symbol,
            style,
            kind,
        }
    }

    pub fn is_walkable(&self) -> bool {
        // not walkable if there is a structure on the tile
        if let Some(_) = &self.structure {
            return false;
        }
        self.kind.is_walkable()
    }

    /// interacts with the structure on the tile
    pub fn interact(&mut self, logger: &mut Logger) {
        if let Some(structure) = &mut self.structure {
            structure.interact(logger);
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
}

impl Drawable for Tile {
    fn draw(
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
            camera.draw_from_screen_coordinates(&self.symbol, self.style(), area.into(), buffer);
        }
    }

    fn symbol(&self) -> &'static str {
        if let Some(structure) = &self.structure {
            structure.symbol()
        } else {
            self.symbol
        }
    }

    fn color(&self) -> Color {
        if let Some(structure) = &self.structure {
            structure.color()
        } else {
            self.style.fg.unwrap_or(Color::Reset)
        }
    }

    fn style(&self) -> Style {
        if let Some(structure) = &self.structure {
            structure.style()
        } else {
            self.style
        }
    }
}

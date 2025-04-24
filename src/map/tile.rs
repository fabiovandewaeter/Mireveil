use ratatui::style::{Color, Style};

#[derive(Clone)]
pub enum TileKind {
    //Wall,
    Grass,
    Water,
}

impl TileKind {
    pub fn symbol(&self) -> &'static str {
        match self {
            //TileKind::Wall => "#",
            TileKind::Grass => ",",
            TileKind::Water => "~",
        }
    }

    pub fn color(&self) -> Color {
        match self {
            //TileKind::Wall => Color::Rgb(150, 150, 150),
            TileKind::Grass => Color::Rgb(0, 102, 0),
            TileKind::Water => Color::Rgb(51, 102, 204),
        }
    }

    pub fn style(&self) -> Style {
        match self {
            //TileKind::Wall => Style::default().fg(self.color()),
            TileKind::Grass => Style::default().fg(self.color()),
            TileKind::Water => Style::default().fg(self.color()),
        }
    }

    pub fn is_walkable(&self) -> bool {
        match self {
            //TileKind::Wall => true,
            _ => true,
        }
    }
}

#[derive(Clone)]
pub struct Tile {
    /// the structure on the tile
    pub structure: Option<Structure>,
    pub symbol: &'static str,
    pub style: Style,
    pub solid: bool,
}

impl Tile {
    pub fn new(kind: TileKind) -> Self {
        let symbol = kind.symbol();
        let style = kind.style();
        let is_walkable = kind.is_walkable();
        Self {
            symbol,
            style,
            is_walkable,
        }
    }
}

use ratatui::style::{Color, Style};

pub enum TileKind {
    Wall,
    Grass,
    Water,
}

impl TileKind {
    pub fn symbol(&self) -> &'static str {
        match self {
            TileKind::Wall => "#",
            TileKind::Grass => "·",
            TileKind::Water => "~",
        }
    }

    pub fn color(&self) -> Color {
        match self {
            TileKind::Wall => Color::Rgb(255, 255, 255),
            TileKind::Grass => Color::Rgb(0, 255, 0),
            TileKind::Water => Color::Rgb(0, 0, 255),
        }
    }

    pub fn style(&self) -> Style {
        match self {
            TileKind::Wall => Style::default().fg(self.color()),
            TileKind::Grass => Style::default().fg(self.color()),
            TileKind::Water => Style::default().fg(self.color()),
        }
    }

    pub fn is_solid(&self) -> bool {
        match self {
            TileKind::Wall => true,
            _ => false,
        }
    }

    pub fn block_sight(&self) -> bool {
        match self {
            TileKind::Wall => true,
            _ => false,
        }
    }
}

pub struct Tile {
    kind: TileKind,
    pub symbol: &'static str,
    pub style: Style,
    pub color: Color,
    pub solid: bool,
    pub block_sight: bool,
}

impl Tile {
    pub fn new(kind: TileKind) -> Self {
        let symbol = kind.symbol();
        let style = kind.style();
        let color = kind.color();
        let solid = kind.is_solid();
        let block_sight = kind.block_sight();
        Self {
            kind,
            symbol,
            style,
            color,
            solid,
            block_sight,
        }
    }
}

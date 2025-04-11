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

    pub fn style(&self) -> Style {
        match self {
            TileKind::Wall => Style::default().fg(Color::White),
            TileKind::Grass => Style::default().fg(Color::Green),
            TileKind::Water => Style::default().fg(Color::Blue),
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
    pub solid: bool,
    pub block_sight: bool,
}

impl Tile {
    pub fn new(kind: TileKind) -> Self {
        let symbol = kind.symbol();
        let style = kind.style();
        let solid = kind.is_solid();
        let block_sight = kind.block_sight();
        Self {
            kind,
            symbol,
            style,
            solid,
            block_sight,
        }
    }
}

use ratatui::{
    layout::Rect,
    style::{Color, Style},
};

const SYMBOL: &'static str = "@";
const SYMBOL_COLOR: Color = Color::Yellow;

pub struct Player {
    pub position: (i32, i32),
    pub symbol: &'static str,
    pub style: Style,
}

impl Player {
    pub fn new(position: (i32, i32)) -> Self {
        Self {
            position,
            symbol: SYMBOL,
            style: Style::default().fg(SYMBOL_COLOR),
        }
    }

    pub fn calculate_camera(&self, area: Rect) -> (i32, i32) {
        (
            self.position.0 as i32 - area.width as i32 / 2,
            self.position.1 as i32 - area.height as i32 / 2,
        )
    }
}

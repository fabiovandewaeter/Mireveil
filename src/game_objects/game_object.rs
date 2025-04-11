use crossterm::event::KeyCode;
use ratatui::{buffer::Buffer, layout::Rect};

use crate::map::map::Map;

pub trait GameObject {
    fn process_key(&mut self, key_code: KeyCode, map: &mut Map);
    fn update(&self);
    fn draw(&self, buffer: &mut Buffer, area: Rect);
}

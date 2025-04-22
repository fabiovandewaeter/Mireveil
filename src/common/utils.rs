use ratatui::{buffer::Buffer, layout::Rect};

pub trait Drawable {
    fn draw(&self, buffer: &mut Buffer, area: Rect, camera_position: (i32, i32));
}

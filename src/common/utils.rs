use ratatui::{buffer::Buffer, layout::Rect};

use crate::systems::camera::Camera;

pub trait Drawable {
    fn draw(&self, buffer: &mut Buffer, area: Rect, camera: &Camera);
}

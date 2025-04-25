use ratatui::{
    buffer::Buffer,
    layout::{Position, Rect},
    style::{Color, Style},
};

use crate::{map::map::Map, systems::camera::Camera};

pub trait Drawable {
    fn draw(&self, buffer: &mut Buffer, area: Rect, camera: &Camera, map: &Map);
    fn draw_at(
        &self,
        position: Position,
        buffer: &mut Buffer,
        area: Rect,
        camera: &Camera,
        map: &Map,
    );
    fn symbol(&self) -> &'static str;
    fn color(&self) -> Color;
    fn style(&self) -> Style;
}

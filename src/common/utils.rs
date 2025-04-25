use ratatui::style::{Color, Style};

pub trait Drawable {
    fn symbol(&self) -> &'static str;
    fn color(&self) -> Color;
    fn style(&self) -> Style {
        Style::default().fg(self.color())
    }
}

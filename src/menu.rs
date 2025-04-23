use ratatui::{
    prelude::*,
    widgets::{Block, Borders, Clear, Paragraph, Wrap},
};

use crate::app::App;

pub struct Logger {
    pub logs: Vec<String>,
    max_displayed_logs: u8,
}

impl Logger {
    pub fn new() -> Self {
        Self {
            logs: Vec::new(),
            max_displayed_logs: 30,
        }
    }

    pub fn push_message(&mut self, msg: String) {
        self.logs.push(msg)
    }
}

pub struct Menu {
    pub visible: bool,
    pub selected_tile_info: Option<String>,
    pub selected_entity_info: Option<String>,
    pub logger: Logger,
}

impl Menu {
    pub fn area(&self, area: Rect) -> Rect {
        let width = (area.width * 3) / 10;
        Rect::new(area.right() - width, area.y, width, area.height)
    }

    pub fn draw(&self, frame: &mut Frame, area: Rect, app: &App) {
        let menu_area = self.area(area);
        // clear the menu_area
        frame.render_widget(Clear, menu_area);

        let block = Block::default()
            .title(" Menu ")
            .borders(Borders::ALL)
            .border_style(Style::new().light_red())
            .title_style(Style::new().white().bold())
            .style(Style::new().bg(Color::Rgb(30, 30, 40)));
        frame.render_widget(block, menu_area);

        // inner area for the texte inside the block
        let inner_area = Rect::new(
            menu_area.x + 1,
            menu_area.y + 1,
            menu_area.width - 2,
            menu_area.height - 2,
        );

        // vector of Line that will be drawn
        let mut lines = Vec::new();

        // add player coordinates at the top
        let (x, y, z) = app.entity_manager.player.position;
        lines.push(Line::from(Span::styled(
            format!("Player: ({}, {}, {})", x, y, z),
            Style::default().fg(Color::Cyan),
        )));

        // draws the informations
        if let Some(ref entity_info) = self.selected_entity_info {
            lines.push(Line::from(Span::styled(
                format!("Entity: {}", entity_info),
                Style::default().fg(Color::Yellow),
            )));
        } else if let Some(ref tile_info) = self.selected_tile_info {
            lines.push(Line::from(Span::styled(
                format!("Tile: {}", tile_info),
                Style::default().fg(Color::Green),
            )));
        } else {
            lines.push(Line::from(Span::styled(
                "No selection",
                Style::default().fg(Color::Gray),
            )));
        }

        // separator
        lines.push(Line::from("────────────".dim()));

        // draw the logs
        lines.push(Line::from(Span::styled(
            "Lastest actions:",
            Style::default().fg(Color::White).bold(),
        )));
        for log in self
            .logger
            .logs
            .iter()
            .rev()
            .take(self.logger.max_displayed_logs as usize)
        {
            lines.push(Line::from(Span::raw(log)));
        }

        // creates and draw the Paragraph of Text
        let paragraph = Paragraph::new(lines).wrap(Wrap { trim: true });
        frame.render_widget(paragraph, inner_area);
    }
}

impl Default for Menu {
    // zone where the widget will be drawn
    fn default() -> Self {
        Self {
            visible: true,
            selected_tile_info: None,
            selected_entity_info: None,
            logger: Logger::new(),
        }
    }
}

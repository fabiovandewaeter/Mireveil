use ratatui::{
    prelude::*,
    text::Text,
    widgets::{Block, Borders, Clear, Paragraph, Wrap},
};

pub struct Logger {
    pub logs: Vec<String>,
}

impl Logger {
    pub fn new() -> Self {
        Self { logs: Vec::new() }
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

    pub fn draw(&self, frame: &mut Frame, area: Rect) {
        // Récupère la zone du menu en fonction de l'aire globale
        let menu_area = self.area(area);
        // Efface la zone pour éviter de laisser des résidus d'un rendu précédent
        frame.render_widget(Clear, menu_area);

        // Crée le block qui sert de cadre au menu
        let block = Block::default()
            .title(" Menu ")
            .borders(Borders::ALL)
            .border_style(Style::new().light_red())
            .title_style(Style::new().white().bold())
            .style(Style::new().bg(Color::Rgb(30, 30, 40)));
        frame.render_widget(block, menu_area);

        // Définir une zone intérieure pour le texte à l'intérieur du block
        let inner_area = Rect::new(
            menu_area.x + 1,
            menu_area.y + 1,
            menu_area.width - 2,
            menu_area.height - 2,
        );

        // Construire un vecteur de Line pour le texte à afficher
        let mut lines: Vec<Line> = Vec::new();

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

        // Convertir le vecteur de Line en Text
        let text = Text::from(lines);

        // Créer et afficher un Paragraph avec le texte
        let paragraph = Paragraph::new(text).wrap(Wrap { trim: true });
        frame.render_widget(paragraph, inner_area);
    }
}

impl Default for Menu {
    // zone where the widget will be drawn
    fn default() -> Self {
        Self {
            visible: false,
            selected_tile_info: None,
            selected_entity_info: None,
            logger: Logger::new(),
        }
    }
}

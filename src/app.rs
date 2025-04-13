use color_eyre::Result;
use crossterm::event::{self, Event, KeyCode, KeyEvent, KeyEventKind};
use ratatui::{
    Terminal,
    backend::CrosstermBackend,
    prelude::*,
    widgets::{Block, Borders, Clear},
};

use crate::{
    game_objects::entity::{Controller, Entity, EntityKind},
    map::map::*,
    systems::entity_manager::EntityManager,
};

#[derive(Clone)]
pub struct Config {
    background_style: Style,
}

impl Default for Config {
    fn default() -> Self {
        Self {
            // brown
            //background_style: Style::default().bg(Color::Rgb(30, 26, 21)),
            // blue
            background_style: Style::default().bg(Color::Rgb(30, 30, 40)),
        }
    }
}

pub struct App {
    map: Map,
    entity_manager: EntityManager,
    config: Config,
    exit: bool,
    inventory_visible: bool,
}

impl App {
    pub fn new(config: Config) -> Self {
        let mut entity_manager = EntityManager::new();
        entity_manager.add_entity(Box::new(Entity::new(
            EntityKind::Dragon,
            (0, 1),
            Controller::AI,
        )));
        entity_manager.add_entity(Box::new(Entity::new(
            EntityKind::Sheep,
            (1, 0),
            Controller::AI,
        )));
        Self {
            map: Map::default(),
            entity_manager: entity_manager,
            config,
            exit: false,
            inventory_visible: false,
        }
    }

    pub fn run(mut self, mut terminal: Terminal<CrosstermBackend<std::io::Stdout>>) -> Result<()> {
        while !self.exit {
            terminal.draw(|f| self.draw(f))?;
            self.handle_events()?;
        }
        Ok(())
    }

    fn handle_events(&mut self) -> Result<()> {
        if let Event::Key(key) = event::read()? {
            if key.kind == KeyEventKind::Press {
                self.process_key(key);
            }
        }
        Ok(())
    }

    fn process_key(&mut self, key: KeyEvent) {
        if key.kind == KeyEventKind::Press {
            match key.code {
                KeyCode::Char('q') => self.exit = true,
                KeyCode::Char('e') => self.inventory_visible = !self.inventory_visible, // Toggle inventaire
                _ => self.entity_manager.update(key.code, &mut self.map),
            }
        }
    }

    pub fn calculate_camera_position(&self, player: &Entity, area: Rect) -> (i32, i32) {
        (
            player.position.0 - (area.width as i32 / 2),
            player.position.1 - (area.height as i32 / 2),
        )
    }

    fn draw_inventory(&self, frame: &mut Frame, area: Rect) {
        // zone where the widget will be drawn
        let width = (area.width * 3) / 10;
        let inventory_area = Rect::new(area.right() - width, area.y, width, area.height);

        // clear the zone on the screen
        frame.render_widget(Clear, inventory_area);

        // Inventory widget
        let block = Block::default()
            .title(" Inventory ")
            .borders(Borders::ALL)
            .border_style(Style::new().light_red())
            .title_style(Style::new().white().bold())
            .style(Style::new().bg(Color::Rgb(30, 30, 40)));

        frame.render_widget(block, inventory_area);

        // TODO: add inventory content
    }

    fn draw(&self, frame: &mut Frame) {
        let area = frame.area();

        // draw background color
        let background = Block::default()
            .style(self.config.background_style)
            .borders(Borders::NONE);
        frame.render_widget(background, area);

        //let (camera_x, camera_y) = self.player.calculate_camera(area);
        let buffer = frame.buffer_mut();

        // draw map
        let (camera_x, camera_y) =
            self.calculate_camera_position(&self.entity_manager.player, area);
        self.map.draw(buffer, area, (camera_x, camera_y));

        // draw entities
        self.entity_manager.draw(buffer, area, (camera_x, camera_y));

        // draw inventory
        if self.inventory_visible {
            self.draw_inventory(frame, area);
        }
    }
}

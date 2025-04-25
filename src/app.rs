use color_eyre::Result;
use crossterm::event::{self, Event, KeyCode, KeyEvent, KeyEventKind, MouseEvent, MouseEventKind};
use ratatui::{
    Terminal,
    backend::CrosstermBackend,
    prelude::*,
    widgets::{Block, Borders},
};

use crate::{
    common::utils::Drawable,
    entities::{
        AIbehavior::ChasePlayerBehavior,
        controller::Controller,
        entity::{Entity, EntityKind},
    },
    map::map::*,
    menu::Menu,
    systems::{
        camera::Camera,
        entity_manager::EntityManager,
        spawner::{Spawner, SpawnerConfiguration},
    },
};

#[derive(Clone)]
pub struct Config {
    background_style: Style,
}

impl Default for Config {
    fn default() -> Self {
        Self {
            // dark brown
            //background_style: Style::default().bg(Color::Rgb(30, 26, 21)),
            // dark blue
            background_style: Style::default().bg(Color::Rgb(30, 30, 40)),
        }
    }
}

pub struct App {
    map: Map,
    pub entity_manager: EntityManager,
    config: Config,
    exit: bool,
    menu: Menu,
    pub camera: Camera,
}

impl App {
    pub fn new(config: Config) -> Self {
        let mut entity_manager = EntityManager::new();
        entity_manager.add_entity(Entity::new(
            EntityKind::Dragon,
            EntityKind::Dragon.name().to_owned(),
            (0, 1, 0),
            Controller::AI(Box::new(ChasePlayerBehavior)),
        ));
        entity_manager.add_entity(Entity::new(
            EntityKind::Sheep,
            EntityKind::Sheep.name().to_owned(),
            (1, 0, 0),
            Controller::AI(Box::new(ChasePlayerBehavior)),
        ));
        Self {
            map: Map::default(),
            entity_manager: entity_manager,
            config,
            exit: false,
            menu: Menu::default(),
            camera: Camera::new((0, 0, 0)),
        }
    }

    pub fn run(mut self, mut terminal: Terminal<CrosstermBackend<std::io::Stdout>>) -> Result<()> {
        let spawner_config = SpawnerConfiguration::default();
        let mut spawner = Spawner::new(spawner_config);
        while !self.exit {
            self.handle_events()?;
            let (cols, rows) = crossterm::terminal::size()?;
            self.update_camera_position(Rect::new(0, 0, cols, rows));
            //self.update();
            spawner.try_spawn(&mut self.entity_manager, &self.map);
            terminal.draw(|f| self.draw(f))?;
        }
        Ok(())
    }

    fn handle_events(&mut self) -> Result<()> {
        match event::read()? {
            Event::Key(key) => {
                if key.kind == KeyEventKind::Press {
                    self.process_key(key);
                }
            }
            Event::Mouse(mouse_event) => {
                self.process_mouse(mouse_event);
            }
            _ => {}
        }
        Ok(())
    }

    fn process_key(&mut self, key: KeyEvent) {
        if key.kind == KeyEventKind::Press {
            match key.code {
                KeyCode::Char('q') => self.exit = true,
                KeyCode::Char('e') => self.menu.visible = !self.menu.visible, // Toggle inventaire
                _ => self.entity_manager.update(
                    key.code,
                    &self.camera,
                    &mut self.map,
                    &mut self.menu.logger,
                ),
            }
        }
    }

    fn process_mouse(&mut self, mouse_event: MouseEvent) {
        match mouse_event.kind {
            MouseEventKind::Down(_) => {
                let click_x = mouse_event.column;
                let click_y = mouse_event.row;

                if let Ok((cols, rows)) = crossterm::terminal::size() {
                    let screen_area = Rect::new(0, 0, cols, rows);
                    let menu_area = self.menu.area(screen_area);

                    // returns if the clic in on the menu pop-up
                    if click_x >= menu_area.x
                        && click_x < menu_area.x + menu_area.width
                        && click_y >= menu_area.y
                        && click_y < menu_area.y + menu_area.height
                    {
                        return;
                    }

                    //let player_position = self.entity_manager.player.position;
                    let player_position = self
                        .entity_manager
                        .get_player_position()
                        .unwrap_or((0, 0, 0));
                    let (camera_x, camera_y) = self
                        .camera
                        .get_center((player_position.0, player_position.1), screen_area);

                    // converts to map coordinates
                    let world_x = camera_x + click_x as i32;
                    let world_y = camera_y + click_y as i32;

                    // try to find the entity at the coordiantes
                    if let Some(entity) =
                        self.entity_manager
                            .find_entity_at((world_x, world_y, player_position.2))
                    {
                        self.menu.selected_entity_info = Some(String::from(entity.symbol()));
                        self.menu.selected_tile_info = None;
                    }
                    // otherwise gets the tile
                    else if let Some(tile) =
                        self.map
                            .get_tile((world_x, world_y, self.camera.position.2))
                    {
                        self.menu.selected_tile_info = Some(String::from(tile.symbol()));
                        self.menu.selected_entity_info = None;
                    } else {
                        self.menu.selected_tile_info = None;
                        self.menu.selected_entity_info = None;
                    }
                }
            }
            _ => {}
        }
    }

    fn update_camera_position(&mut self, area: Rect) {
        //let player_pos = self.entity_manager.player.position;
        let player_position = self
            .entity_manager
            .get_player_position()
            .unwrap_or((0, 0, 0));
        self.camera.position.0 = player_position.0 - (area.width as i32 / 2);
        self.camera.position.1 = player_position.1 - (area.height as i32 / 2);
        self.camera.position.2 = player_position.2;
    }

    fn draw(&self, frame: &mut Frame) {
        let area = frame.area();

        // draws background color
        let background = Block::default()
            .style(self.config.background_style)
            .borders(Borders::NONE);
        frame.render_widget(background, area);

        let buffer = frame.buffer_mut();

        // draws map
        self.map.draw(buffer, area, &self.camera, &self.map);

        // draws entities
        self.entity_manager
            .draw(buffer, area, &self.camera, &self.map);

        // draws menu
        if self.menu.visible {
            self.menu.draw(frame, area, &self);
        }
    }
}

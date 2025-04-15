use crossterm::event::KeyCode;
use ratatui::{
    buffer::Buffer,
    layout::{Position, Rect},
    style::{Color, Style},
};

use crate::{
    map::map::{CHUNK_SIZE, Map},
    menu::Logger,
};

pub trait Drawable {
    fn draw(&self, buffer: &mut Buffer, area: Rect, camera_position: (i32, i32));
}

pub enum EntityKind {
    Human,
    Dragon,
    Sheep,
}

impl EntityKind {
    fn symbol(&self) -> &'static str {
        match self {
            EntityKind::Human => "@",
            EntityKind::Dragon => "D",
            EntityKind::Sheep => "S",
        }
    }

    fn style(&self) -> Style {
        match self {
            EntityKind::Human => Style::default().fg(Color::Rgb(255, 255, 255)),
            EntityKind::Dragon => Style::default().fg(Color::Rgb(255, 0, 0)),
            EntityKind::Sheep => Style::default().fg(Color::Rgb(255, 209, 223)),
        }
    }

    fn stats(&self) -> EntityStats {
        match self {
            EntityKind::Human => EntityStats {
                max_hp: 100,
                hp: 100,
                max_mana: 100,
                mana: 100,
                defense: 5,
                strength: 5,
                magic: 5,
            },
            EntityKind::Dragon => EntityStats {
                max_hp: 1000,
                hp: 1000,
                max_mana: 200,
                mana: 200,
                defense: 50,
                strength: 50,
                magic: 20,
            },
            EntityKind::Sheep => EntityStats {
                max_hp: 30,
                hp: 30,
                max_mana: 0,
                mana: 0,
                defense: 5,
                strength: 1,
                magic: 0,
            },
            _ => EntityStats::default(),
        }
    }

    fn actions(&self) -> Vec<Box<dyn EntityAction>> {
        match self {
            EntityKind::Human => {
                vec![Box::new(Attack::new(10))]
            }
            EntityKind::Dragon => {
                vec![Box::new(Attack::new(10))]
            }
            EntityKind::Sheep => {
                vec![Box::new(Attack::new(10))]
            }
        }
    }
}

// who controls the Entity
#[derive(Copy, Clone)]
pub enum Controller {
    Player,
    AI,
}

impl Controller {
    pub fn update_entity<'a, I>(
        &self,
        entity: &mut Entity,
        input: Option<KeyCode>,
        map: &mut Map,
        other_entities: I,
        logger: &mut Logger,
    ) where
        I: Iterator<Item = &'a mut Entity>,
    {
        match self {
            Controller::Player => {
                if let Some(key_code) = input {
                    self.handle_player_input(entity, key_code, map, other_entities, logger);
                }
            }
            Controller::AI => {}
        }
    }

    fn handle_player_input<'a, I>(
        &self,
        entity: &mut Entity,
        key_code: KeyCode,
        map: &mut Map,
        other_entities: I,
        logger: &mut Logger,
    ) where
        I: Iterator<Item = &'a mut Entity>,
    {
        let (dx, dy) = match key_code {
            KeyCode::Up => (0, -1),
            KeyCode::Down => (0, 1),
            KeyCode::Left => (-1, 0),
            KeyCode::Right => (1, 0),
            _ => (0, 0),
        };

        let new_x = entity.position.0 + dx;
        let new_y = entity.position.1 + dy;

        self.handle_entity_movement(entity, new_x, new_y, map, other_entities, logger);
    }

    fn handle_entity_movement<'a, I>(
        &self,
        entity: &mut Entity,
        new_x: i32,
        new_y: i32,
        map: &mut Map,
        mut other_entities: I,
        logger: &mut Logger,
    ) where
        I: Iterator<Item = &'a mut Entity>,
    {
        if let Some(target) = other_entities.find(|e| e.position == (new_x, new_y)) {
            // attack the entity
            for action in &entity.actions {
                if let Some(msg) = action.affect(entity, target) {
                    logger.push_message(msg);
                }
            }
        }
        if let Some(tile) = map.get_tile(new_x, new_y) {
            if !tile.solid {
                entity.position = (new_x, new_y);
                map.load_around((
                    new_x.div_euclid(CHUNK_SIZE as i32),
                    new_y.div_euclid(CHUNK_SIZE as i32),
                ));
            }
        }
    }
}

#[derive(Default)]
pub struct EntityStats {
    pub max_hp: u32,
    pub hp: u32,
    pub max_mana: u32,
    pub mana: u32,
    pub defense: u32,
    pub strength: u32,
    pub magic: u32,
}

pub struct Entity {
    kind: EntityKind,
    pub position: (i32, i32),
    pub symbol: &'static str,
    pub style: Style,
    pub controller: Controller,
    pub stats: EntityStats,
    actions: Vec<Box<dyn EntityAction>>,
}

impl Entity {
    pub fn new(kind: EntityKind, position: (i32, i32), controller: Controller) -> Self {
        Self {
            position,
            symbol: kind.symbol(),
            style: kind.style(),
            controller,
            stats: kind.stats(),
            actions: kind.actions(),
            kind,
        }
    }

    pub fn player(position: (i32, i32)) -> Self {
        Self::new(EntityKind::Human, position, Controller::Player)
    }

    pub fn update<'a, I>(
        &mut self,
        input: Option<KeyCode>,
        map: &mut Map,
        other_entities: I,
        logger: &mut Logger,
    ) where
        I: Iterator<Item = &'a mut Entity>,
    {
        let controller = self.controller;
        controller.update_entity(self, input, map, other_entities, logger);
    }
}

impl Drawable for Entity {
    fn draw(&self, buffer: &mut Buffer, area: Rect, camera_position: (i32, i32)) {
        let screen_x = self.position.0 - camera_position.0;
        let screen_y = self.position.1 - camera_position.1;

        // only draw if the Entity is close enough to the camera
        if screen_x >= 0
            && screen_x < area.width as i32
            && screen_y >= 0
            && screen_y < area.height as i32
        {
            let position: Position = Position {
                x: screen_x as u16,
                y: screen_y as u16,
            };
            let cell = buffer.cell_mut(position).unwrap();
            cell.set_symbol(self.symbol);
            cell.set_style(self.style);
        }
    }
}

trait EntityAction {
    fn affect(&self, source: &Entity, target: &mut Entity) -> Option<String>;
}

struct Attack {
    damage: u32,
    // TODO: type, range etc.
}

impl Attack {
    fn new(damage: u32) -> Self {
        Self { damage }
    }
}

impl EntityAction for Attack {
    fn affect(&self, source: &Entity, target: &mut Entity) -> Option<String> {
        // makes sure the target dont get negative hp
        let damage = std::cmp::min(self.damage, target.stats.hp);
        target.stats.hp -= damage;
        //print!("attack {} : {} damage", target.symbol, damage_applied);
        Some(format!(
            "{} attacks {} (-{} PV)",
            source.symbol, target.symbol, damage
        ))
    }
}

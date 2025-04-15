use crossterm::event::KeyCode;
use ratatui::{
    buffer::Buffer,
    layout::{Position, Rect},
    style::{Color, Style},
};

use crate::{
    entities::action::Attack,
    map::map::{CHUNK_SIZE, Map},
    menu::Logger,
    systems::level_manager::LevelManager,
};

use super::action::Action;

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

    fn actions(&self) -> Vec<Box<dyn Action>> {
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
            let target_was_alive = !target.is_dead();
            // attack the entity
            for action in &entity.actions {
                if let Some(msg) = action.affect(entity, target) {
                    logger.push_message(msg);
                }
            }
            // if the target is now dead
            if target_was_alive && target.is_dead() {
                logger.push_message(format!(
                    "{} xp needed for next level",
                    entity.level_manager.xp_to_next_level()
                ));
                Self::handle_xp_gain(entity, target, logger);
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

    fn handle_xp_gain(attacker: &mut Entity, target: &mut Entity, logger: &mut Logger) {
        let xp_gained = target.xp_drop;

        let levels_gained = target.level_manager.add_xp(xp_gained);
        logger.push_message(format!("{} gained {} XP", attacker.symbol(), xp_gained));

        if levels_gained > 0 {
            logger.push_message(format!(
                "{} reached level {}",
                attacker.symbol(),
                attacker.level_manager.level
            ));
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
    controller: Controller,
    pub stats: EntityStats,
    xp_drop: u32,
    level_manager: LevelManager,
    actions: Vec<Box<dyn Action>>,
}

impl Entity {
    pub fn new(kind: EntityKind, position: (i32, i32), controller: Controller) -> Self {
        Self {
            position,
            controller,
            stats: kind.stats(),
            xp_drop: 10,
            level_manager: LevelManager::default(),
            actions: kind.actions(),
            kind,
        }
    }

    pub fn symbol(&self) -> &'static str {
        self.kind.symbol()
    }

    pub fn style(&self) -> Style {
        self.kind.style()
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

    pub fn take_damage(&mut self, amount: u32) -> u32 {
        let damage = std::cmp::min(amount, self.stats.hp);
        self.stats.hp -= damage;
        damage
    }

    pub fn is_dead(&self) -> bool {
        self.stats.hp == 0
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
            cell.set_symbol(self.symbol());
            cell.set_style(self.style());
        }
    }
}

use std::{
    collections::{HashMap, HashSet},
    hash::Hash,
};

use ratatui::{
    buffer::Buffer,
    layout::{self, Position, Rect},
    style::{Color, Style},
};

use crate::systems::camera::style_to_greyscale;

use super::tile::{Tile, TileKind};

/// size in tiles
pub const CHUNK_SIZE: u16 = 32;
/// distance in chunk chunks are loaded
pub const LOAD_DISTANCE: i32 = 2;

/// a Chunk is made up of several layers of CHUNK_SIZE*CHUNK_SIZE tiles
pub struct Chunk {
    /// layers are squares of CHUNK_SIZE*CHUNK_SIZE tiles
    pub layers: HashMap<i32, Vec<Vec<Tile>>>,
    pub position: (i32, i32),
}

impl Chunk {
    /// creates a new Chunk with one layer already loaded
    pub fn new(chunk_x: i32, chunk_y: i32, layer: i32) -> Self {
        let tiles = (0..CHUNK_SIZE)
            .map(|x| {
                (0..CHUNK_SIZE)
                    .map(|y| {
                        if x % 32 == 0 && y % 32 == 0 {
                            Tile::new(TileKind::Wall)
                        } else if x % 32 == 1 && y % 32 == 1 {
                            Tile::new(TileKind::Water)
                        } else {
                            Tile::new(TileKind::Grass)
                        }
                    })
                    .collect()
            })
            .collect();

        let mut layers = HashMap::new();
        layers.insert(layer, tiles);
        Self {
            layers,
            position: (chunk_x, chunk_y),
        }
    }

    pub fn get_tile(&self, global_x: i32, global_y: i32, layer: i32) -> Option<&Tile> {
        let (local_x, local_y) = Self::convert_to_local_chunk_coordinates(global_x, global_y);
        self.layers
            .get(&layer)
            .and_then(|tiles| tiles.get(local_y))
            .and_then(|row| row.get(local_x))
    }

    /// converts global tiles coordinates to chunk-local coordiantes
    pub fn convert_to_local_chunk_coordinates(global_x: i32, global_y: i32) -> (usize, usize) {
        (
            global_x.rem_euclid(CHUNK_SIZE as i32) as usize,
            global_y.rem_euclid(CHUNK_SIZE as i32) as usize,
        )
    }
}

pub struct Map {
    pub chunks: HashMap<(i32, i32), Chunk>,
    visible_layer: i32,
    /// coordinates of tiles that will be drawn
    pub visible_tiles: HashSet<(i32, i32)>,
    /// coordinates of tiles that were seen and will be drawn in black and white
    pub revealed_tiles: HashSet<(i32, i32)>,
}

impl Map {
    pub fn new() -> Self {
        Self {
            chunks: HashMap::new(),
            visible_layer: 0,
            visible_tiles: HashSet::new(),
            revealed_tiles: HashSet::new(),
        }
    }

    /// generates one chunk at (x, y) in chunk coordinates with one layer of tiles at the current visible layer
    pub fn load_chunk(&mut self, chunk_x: i32, chunk_y: i32) {
        self.chunks
            .entry((chunk_x, chunk_y))
            .or_insert_with(|| Chunk::new(chunk_x, chunk_y, self.visible_layer));
    }

    /// generates chunks around the point
    pub fn load_around(&mut self, point: (i32, i32)) {
        for y in (point.1 - LOAD_DISTANCE)..=(point.1 + LOAD_DISTANCE) {
            for x in (point.0 - LOAD_DISTANCE)..=(point.0 + LOAD_DISTANCE) {
                self.chunks
                    .entry((x, y))
                    .or_insert_with(|| Chunk::new(x, y, self.visible_layer));
            }
        }
    }

    pub fn get_tile(&self, global_x: i32, global_y: i32, layer: i32) -> Option<&Tile> {
        let (chunk_x, chunk_y) = Self::convert_to_chunk_coordinates(global_x, global_y);
        self.chunks
            .get(&(chunk_x, chunk_y))
            .and_then(|chunk| chunk.get_tile(global_x, global_y, layer))
    }

    /// draws the tiles of the map only for the visible layer
    pub fn draw(&self, buffer: &mut Buffer, area: Rect, camera_position: (i32, i32)) {
        for screen_y in 0..area.height {
            for screen_x in 0..area.width {
                let world_x = camera_position.0 + screen_x as i32;
                let world_y = camera_position.1 + screen_y as i32;
                let pos = (world_x, world_y);
                let (symbol, style) =
                    if let Some(tile) = self.get_tile(world_x, world_y, self.visible_layer) {
                        if self.visible_tiles.contains(&pos) {
                            // if tile is visible, use its symbol
                            (tile.symbol, tile.style)
                        } else if self.revealed_tiles.contains(&pos) {
                            // if tile is not visible, use grayed-out version of its symbol
                            (
                                tile.symbol,
                                tile.style.patch(style_to_greyscale(tile.color)),
                            )
                        } else {
                            // if not revealed, draw empty tile
                            (" ", Style::default())
                        }
                    } else {
                        ("#", Style::default().fg(Color::Red))
                    };

                let position: Position = Position {
                    x: screen_x,
                    y: screen_y,
                };
                let cell = buffer.cell_mut(position).unwrap();
                cell.set_symbol(symbol);
                cell.set_style(style);
            }
        }
    }

    pub fn convert_to_chunk_coordinates(global_x: i32, global_y: i32) -> (i32, i32) {
        (
            global_x.div_euclid(CHUNK_SIZE as i32),
            global_y.div_euclid(CHUNK_SIZE as i32),
        )
    }
}

impl Default for Map {
    fn default() -> Self {
        let mut map = Self::new();
        // 3x3 chunks
        for x in -1..=1 {
            for y in -1..=1 {
                map.load_chunk(x, y);
            }
        }
        map
    }
}

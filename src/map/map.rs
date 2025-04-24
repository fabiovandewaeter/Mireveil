use std::collections::{HashMap, HashSet};

use ratatui::{buffer::Buffer, layout::Rect};

use crate::{common::utils::Drawable, systems::camera::Camera};

use super::tile::{Tile, TileKind};

/// size in tiles
pub const CHUNK_SIZE: u16 = 32;
/// distance in chunk chunks are loaded
pub const LOAD_DISTANCE: i32 = 2;

/// layers are squares of CHUNK_SIZE*CHUNK_SIZE tiles
pub struct Layer {
    tiles: Vec<Vec<Tile>>,
    position: (i32, i32),
    /// coordinates of tiles that will be drawn
    pub visible_tiles: HashSet<(i32, i32)>,
    /// coordinates of tiles that were seen and will be drawn in black and white
    pub revealed_tiles: HashSet<(i32, i32)>,
}

impl Layer {
    pub fn new(tiles: Vec<Vec<Tile>>, position: (i32, i32)) -> Self {
        Self {
            tiles,
            position,
            visible_tiles: HashSet::new(),
            revealed_tiles: HashSet::new(),
        }
    }
}

impl Drawable for Layer {
    fn draw(&self, buffer: &mut Buffer, area: Rect, camera: &Camera, _map: &Map) {
        let (chunk_world_x, chunk_world_y) = self.position;

        // merge visible and revealed sets
        let all_tiles = self.visible_tiles.union(&self.revealed_tiles);

        for &(global_x, global_y) in all_tiles {
            // compute local indices
            let local_x = (global_x - chunk_world_x) as usize;
            let local_y = (global_y - chunk_world_y) as usize;
            if local_x >= CHUNK_SIZE.into() || local_y >= CHUNK_SIZE.into() {
                continue;
            }

            // project to screen coordinates
            if let Some((buf_x, buf_y)) = camera.world_to_screen((global_x, global_y), area) {
                let tile = &self.tiles[local_y][local_x];
                let is_visible = self.visible_tiles.contains(&(global_x, global_y));
                let style = camera.grays_tile_if_not_visible(tile, is_visible);

                camera.draw_from_screen_coordinates(
                    &tile.symbol,
                    style,
                    (buf_x, buf_y).into(),
                    buffer,
                );
            }
        }
    }
}

/// a Chunk is made up of several layers of CHUNK_SIZE*CHUNK_SIZE tiles
pub struct Chunk {
    pub layers: HashMap<i32, Layer>,
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
                        } else if x % 32 == 2 && y % 32 == 2 {
                            Tile::new(TileKind::Water)
                        } else {
                            Tile::new(TileKind::Grass)
                        }
                    })
                    .collect()
            })
            .collect();

        let world_x = chunk_x * CHUNK_SIZE as i32;
        let world_y = chunk_y * CHUNK_SIZE as i32;

        let mut layers = HashMap::new();
        layers.insert(layer, Layer::new(tiles, (world_x, world_y)));
        Self {
            layers,
            position: (chunk_x, chunk_y),
        }
    }

    /// returns the tile from global coorinates
    pub fn get_tile(&self, global_coordinates: (i32, i32, i32)) -> Option<&Tile> {
        let (local_x, local_y) =
            Self::convert_to_local_chunk_coordinates(global_coordinates.0, global_coordinates.1);
        self.layers
            .get(&global_coordinates.2)
            .and_then(|layer| layer.tiles.get(local_y).and_then(|row| row.get(local_x)))
    }

    /// returns global coordinates for left corner of the chunk
    pub fn get_chunk_world_position(&self) -> (i32, i32) {
        (
            self.position.0 * CHUNK_SIZE as i32,
            self.position.1 * CHUNK_SIZE as i32,
        )
    }

    /// returns true if the chunk is visible by the camera
    pub fn is_visible(&self, area: Rect, camera: &Camera) -> bool {
        let top_left_position = self.get_chunk_world_position();
        let dimensions = (CHUNK_SIZE as i32, CHUNK_SIZE as i32);

        camera.is_rect_on_screen(
            (top_left_position.0, top_left_position.1, camera.position.2),
            dimensions,
            area,
        )
    }

    /// converts global tiles coordinates to chunk-local coordiantes
    pub fn convert_to_local_chunk_coordinates(global_x: i32, global_y: i32) -> (usize, usize) {
        (
            global_x.rem_euclid(CHUNK_SIZE as i32) as usize,
            global_y.rem_euclid(CHUNK_SIZE as i32) as usize,
        )
    }
}

impl Drawable for Chunk {
    fn draw(&self, buffer: &mut Buffer, area: Rect, camera: &Camera, map: &Map) {
        if let Some(layer) = self.layers.get(&camera.position.2) {
            layer.draw(buffer, area, camera, map);
        }
    }
}

pub struct Map {
    pub chunks: HashMap<(i32, i32), Chunk>,
}

impl Map {
    pub fn new() -> Self {
        Self {
            chunks: HashMap::new(),
        }
    }

    /// generates one chunk at (x, y) in chunk coordinates with one layer of tiles at the current visible layer
    pub fn load_chunk(&mut self, chunk_x: i32, chunk_y: i32, layer: i32) {
        // finds the chunk or generates it
        let chunk = self
            .chunks
            .entry((chunk_x, chunk_y))
            .or_insert_with(|| Chunk::new(chunk_x, chunk_y, layer));

        // add a layer to the chunk if it doesn't exist yet
        if !chunk.layers.contains_key(&layer) {
            let world_x = chunk_x * CHUNK_SIZE as i32;
            let world_y = chunk_y * CHUNK_SIZE as i32;
            let tiles = (0..CHUNK_SIZE)
                .map(|y| {
                    (0..CHUNK_SIZE)
                        .map(|x| {
                            if x == 0 && y == 0 {
                                Tile::new(TileKind::Wall)
                            } else if x == 1 && y == 1 {
                                Tile::new(TileKind::Water)
                            } else {
                                Tile::new(TileKind::Grass)
                            }
                        })
                        .collect()
                })
                .collect();
            chunk
                .layers
                .insert(layer, Layer::new(tiles, (world_x, world_y)));
        }
    }

    /// generates chunks around the global_coordinates
    pub fn load_around(&mut self, global_coordinates: (i32, i32, i32)) {
        for y in (global_coordinates.1 - LOAD_DISTANCE)..=(global_coordinates.1 + LOAD_DISTANCE) {
            for x in (global_coordinates.0 - LOAD_DISTANCE)..=(global_coordinates.0 + LOAD_DISTANCE)
            {
                self.load_chunk(x, y, global_coordinates.2);
            }
        }
    }

    /// returns the tile from global coorinates
    pub fn get_tile(&self, global_coordinates: (i32, i32, i32)) -> Option<&Tile> {
        let (chunk_x, chunk_y) =
            Self::convert_to_chunk_coordinates(global_coordinates.0, global_coordinates.1);
        self.chunks
            .get(&(chunk_x, chunk_y))
            .and_then(|chunk| chunk.get_tile(global_coordinates))
    }

    /// returns chunks potentially visible to the camera
    pub fn get_visible_chunks(&self, area: Rect, camera: &Camera) -> Vec<&Chunk> {
        // Déterminer les chunks qui sont potentiellement visibles
        self.chunks
            .values()
            .filter(|chunk| chunk.is_visible(area, camera))
            .collect()
    }

    /// converts global coordinates to chunk coordinates
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
                map.load_chunk(x, y, 0);
            }
        }
        map
    }
}

impl Drawable for Map {
    /// draws the tiles of the map only for the visible layer
    fn draw(&self, buffer: &mut Buffer, area: Rect, camera: &Camera, map: &Map) {
        let visibile_chunks = self.get_visible_chunks(area, camera);
        for chunk in visibile_chunks {
            chunk.draw(buffer, area, camera, map);
        }
    }
}

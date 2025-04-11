use std::collections::HashMap;

use super::tile::{Tile, TileKind};

// size in chunks
pub const MAP_SIZE: (i32, i32) = (10, 10);
// size in tiles
pub const CHUNK_SIZE: u16 = 32;
// distance in chunk chunks are loaded
pub const LOAD_DISTANCE: i32 = 2;

pub struct Chunk {
    pub tiles: Vec<Vec<Tile>>,
    pub position: (i32, i32),
}

impl Chunk {
    pub fn new(chunk_x: i32, chunk_y: i32) -> Self {
        let tiles = (0..CHUNK_SIZE)
            .map(|x| {
                (0..CHUNK_SIZE)
                    .map(|y| {
                        if x % 10 == 0 && y % 5 == 0 {
                            Tile::new(TileKind::Wall)
                        } else {
                            Tile::new(TileKind::Grass)
                        }
                    })
                    .collect()
            })
            .collect();

        Self {
            tiles,
            position: (chunk_x, chunk_y),
        }
    }
}

/*impl Default for Chunk {
    fn default() -> Self {
        let mut chunk = Self::new();
        // add walls
        for y in 0..CHUNK_SIZE {
            for x in 0..CHUNK_SIZE {
                if x == 0 || y == 0 || x == CHUNK_SIZE - 1 || y == CHUNK_SIZE - 1 {
                    chunk.tiles[y as usize][x as usize] = Tile::new(TileKind::Wall);
                }
            }
        }
        chunk
    }
}*/

pub struct Map {
    pub chunks: HashMap<(i32, i32), Chunk>,
    pub size: (i32, i32),
}

impl Map {
    pub fn new() -> Self {
        Self {
            chunks: HashMap::new(),
            size: MAP_SIZE,
        }
    }

    // generate one chunk at (x, y) in chunk coordinates
    pub fn load_chunk(&mut self, chunk_x: i32, chunk_y: i32) {
        self.chunks
            .entry((chunk_x, chunk_y))
            .or_insert_with(|| Chunk::new(chunk_x, chunk_y));
    }

    pub fn load_around(&mut self, center: (i32, i32)) {
        for y in (center.1 - LOAD_DISTANCE)..=(center.1 + LOAD_DISTANCE) {
            for x in (center.0 - LOAD_DISTANCE)..=(center.0 + LOAD_DISTANCE) {
                self.chunks
                    .entry((x, y))
                    .or_insert_with(|| Chunk::new(x, y));
            }
        }
    }

    pub fn get_tile(&self, global_x: i32, global_y: i32) -> Option<&Tile> {
        let chunk_x = global_x.div_euclid(CHUNK_SIZE as i32);
        let chunk_y = global_y.div_euclid(CHUNK_SIZE as i32);

        let local_x = global_x.rem_euclid(CHUNK_SIZE as i32) as usize;
        let local_y = global_y.rem_euclid(CHUNK_SIZE as i32) as usize;

        self.chunks
            .get(&(chunk_x, chunk_y))
            .and_then(|chunk| chunk.tiles.get(local_y))
            .and_then(|row| row.get(local_x))
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

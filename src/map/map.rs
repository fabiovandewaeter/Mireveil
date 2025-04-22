use std::collections::{HashMap, HashSet};

use ratatui::{buffer::Buffer, layout::Rect};

use crate::{common::utils::Drawable, systems::camera::Camera};

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
    /// coordinates of tiles that will be drawn
    pub visible_tiles: HashSet<(i32, i32)>,
    /// coordinates of tiles that were seen and will be drawn in black and white
    pub revealed_tiles: HashSet<(i32, i32)>,
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
            visible_tiles: HashSet::new(),
            revealed_tiles: HashSet::new(),
        }
    }

    pub fn get_tile(&self, global_x: i32, global_y: i32, layer: i32) -> Option<&Tile> {
        let (local_x, local_y) = Self::convert_to_local_chunk_coordinates(global_x, global_y);
        self.layers
            .get(&layer)
            .and_then(|tiles| tiles.get(local_y))
            .and_then(|row| row.get(local_x))
    }

    /// Calcule les coordonnées globales du coin supérieur gauche du chunk
    pub fn get_chunk_world_position(&self) -> (i32, i32) {
        (
            self.position.0 * CHUNK_SIZE as i32,
            self.position.1 * CHUNK_SIZE as i32,
        )
    }

    pub fn is_visible(&self, area: Rect, camera: &Camera) -> bool {
        let top_left_position = self.get_chunk_world_position();
        let dimensions = (CHUNK_SIZE as i32, CHUNK_SIZE as i32);

        camera.is_rect_on_screen(
            (
                top_left_position.0,
                top_left_position.1,
                camera.visible_layer,
            ),
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
    fn draw(&self, buffer: &mut Buffer, area: Rect, camera: &Camera) {
        // Vérifier si la couche actuelle existe dans ce chunk
        let current_layer = camera.visible_layer;
        let Some(layer_tiles) = self.layers.get(&current_layer) else {
            return;
        };

        // Calculer la position globale du coin supérieur gauche du chunk
        let chunk_world_x = self.position.0 * CHUNK_SIZE as i32;
        let chunk_world_y = self.position.1 * CHUNK_SIZE as i32;

        // Parcourir toutes les tuiles du chunk
        for (local_y, row) in layer_tiles.iter().enumerate() {
            for (local_x, tile) in row.iter().enumerate() {
                // Calculer les coordonnées globales de la tuile
                let global_x = chunk_world_x + local_x as i32;
                let global_y = chunk_world_y + local_y as i32;

                // Vérifier la visibilité et la révélation
                let is_visible = self.visible_tiles.contains(&(global_x, global_y));
                let is_revealed = self.revealed_tiles.contains(&(global_x, global_y));

                if !is_visible && !is_revealed {
                    continue;
                }

                // Calculer la position relative à la caméra
                let screen_x = global_x - camera.position.0;
                let screen_y = global_y - camera.position.1;

                // Vérifier si dans les limites de l'écran
                if screen_x < 0
                    || screen_x >= area.width as i32
                    || screen_y < 0
                    || screen_y >= area.height as i32
                {
                    continue;
                }

                // Déterminer le style
                let style = if is_visible {
                    tile.style
                } else {
                    // Appliquer la conversion en niveaux de gris
                    let mut revealed_style = tile.style;
                    if let Some(fg) = revealed_style.fg {
                        revealed_style.fg = Some(Camera::style_to_greyscale(fg));
                    }
                    revealed_style
                };

                // Calculer la position dans le buffer
                let buffer_x = area.x + screen_x as u16;
                let buffer_y = area.y + screen_y as u16;

                // Vérifier les limites du buffer
                if buffer_x >= area.right() || buffer_y >= area.bottom() {
                    continue;
                }

                // Mettre à jour la cellule du buffer
                let cell = buffer.cell_mut((buffer_x, buffer_y)).unwrap();
                cell.set_symbol(&tile.symbol);
                cell.set_style(style);
            }
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
        self.chunks
            .entry((chunk_x, chunk_y))
            .or_insert_with(|| Chunk::new(chunk_x, chunk_y, layer));
    }

    /// generates chunks around the point
    pub fn load_around(&mut self, point: (i32, i32, i32)) {
        for y in (point.1 - LOAD_DISTANCE)..=(point.1 + LOAD_DISTANCE) {
            for x in (point.0 - LOAD_DISTANCE)..=(point.0 + LOAD_DISTANCE) {
                self.load_chunk(x, y, point.2);
            }
        }
    }

    pub fn get_tile(&self, global_x: i32, global_y: i32, layer: i32) -> Option<&Tile> {
        let (chunk_x, chunk_y) = Self::convert_to_chunk_coordinates(global_x, global_y);
        self.chunks
            .get(&(chunk_x, chunk_y))
            .and_then(|chunk| chunk.get_tile(global_x, global_y, layer))
    }

    /// returns chunks potentially visible to the camera
    pub fn get_visible_chunks(&self, area: Rect, camera: &Camera) -> Vec<&Chunk> {
        // Déterminer les chunks qui sont potentiellement visibles
        self.chunks
            .values()
            .filter(|chunk| chunk.is_visible(area, camera))
            .collect()
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
                map.load_chunk(x, y, 0);
            }
        }
        map
    }
}

impl Drawable for Map {
    /// draws the tiles of the map only for the visible layer
    fn draw(&self, buffer: &mut Buffer, area: Rect, camera: &Camera) {
        let visibile_chunks = self.get_visible_chunks(area, camera);
        for chunk in visibile_chunks {
            chunk.draw(buffer, area, camera);
        }
    }
}

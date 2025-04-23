use std::collections::HashSet;

use ratatui::{
    layout::Rect,
    style::{Color, Style},
};

use crate::map::{map::Map, tile::Tile};

pub struct Camera {
    pub position: (i32, i32, i32),
}

impl Camera {
    pub fn new(starting_position: (i32, i32, i32)) -> Camera {
        Self {
            position: starting_position,
        }
    }

    /// updates the visible_tiles of the map based on the player position
    pub fn update_visibility(&self, player_position: (i32, i32, i32), range: i32, map: &mut Map) {
        // computes FOV for player
        let visible = self.compute_fov(player_position, range, map);

        // reset visible tiles
        for chunk in map.chunks.values_mut() {
            for layer in chunk.layers.values_mut() {
                layer.visible_tiles.clear();
            }
        }

        // updates visible tiles
        for (global_x, global_y) in visible {
            let chunk_coords = Map::convert_to_chunk_coordinates(global_x, global_y);
            if let Some(chunk) = map.chunks.get_mut(&chunk_coords) {
                if let Some(layer) = chunk.layers.get_mut(&player_position.2) {
                    layer.visible_tiles.insert((global_x, global_y));
                    layer.revealed_tiles.insert((global_x, global_y));
                }
            }
        }
    }

    /// private method to update visibility; returns the set of coordinates of the tiles visible to the player
    fn compute_fov(
        &self,
        player_position: (i32, i32, i32),
        range: i32,
        map: &Map,
    ) -> HashSet<(i32, i32)> {
        let mut visible = HashSet::new();
        for y in (player_position.1 - range)..=(player_position.1 + range) {
            for x in (player_position.0 - range)..=(player_position.0 + range) {
                let dx = x - player_position.0;
                let dy = y - player_position.1;
                if dx * dx + dy * dy <= range * range {
                    if self.in_line_of_sight(player_position, (x, y), map) {
                        visible.insert((x, y));
                    }
                }
            }
        }
        visible
    }

    /// returns a grayed-out version of the RGB color
    pub fn style_to_greyscale(color: Color) -> Color {
        match color {
            Color::Rgb(r, g, b) => {
                let grey = ((r as u16 + g as u16 + b as u16) / 3) as u8;
                Color::Rgb(grey, grey, grey)
            }
            _ => Color::Gray, // if not RGB return Grey
        }
    }

    /// styles a tile depending on visibility
    pub fn grays_tile_if_not_visible(&self, tile: &Tile, is_visible: bool) -> Style {
        if is_visible {
            tile.style
        } else {
            let mut gs = tile.style;
            if let Some(fg) = gs.fg {
                gs.fg = Some(Self::style_to_greyscale(fg));
            }
            gs
        }
    }

    /// retuns the camera coordinates so that the center is pointed at the player
    pub fn get_center(&self, player_position: (i32, i32), area: Rect) -> (i32, i32) {
        (
            player_position.0 - (area.width as i32 / 2),
            player_position.1 - (area.height as i32 / 2),
        )
    }

    /// converts a world position to buffer coordinates if on-screen
    pub fn world_to_screen(&self, global: (i32, i32), area: Rect) -> Option<(u16, u16)> {
        let screen_x = global.0 - self.position.0;
        let screen_y = global.1 - self.position.1;
        if screen_x < 0
            || screen_y < 0
            || screen_x >= area.width as i32
            || screen_y >= area.height as i32
        {
            None
        } else {
            Some((area.x + screen_x as u16, area.y + screen_y as u16))
        }
    }

    /// returns true if the point is visible by the camera, false otherwise
    pub fn is_point_on_screen(&self, global_position: (i32, i32, i32), area: Rect) -> bool {
        self.world_to_screen((global_position.0, global_position.1), area) != None
    }

    /// returns true if the rect is visible by the camera, false otherwise
    pub fn is_rect_on_screen(
        &self,
        top_left_coordinates: (i32, i32, i32),
        dimensions: (i32, i32),
        area: Rect,
    ) -> bool {
        // rectangle bounds in world coordinates
        let (left, top) = (top_left_coordinates.0, top_left_coordinates.1);
        let right = left + dimensions.0;
        let bottom = top + dimensions.1;

        // screen bounds in world coordinates
        let screen_left = self.position.0;
        let screen_top = self.position.1;
        let screen_right = self.position.0 + area.width as i32;
        let screen_bottom = self.position.1 + area.height as i32;

        // check for overlap between the two rectangles
        !(right <= screen_left
            || left >= screen_right
            || bottom <= screen_top
            || top >= screen_bottom)
    }

    pub fn is_visible_tile(&self, position: (i32, i32, i32), map: &Map) -> bool {
        let (x, y, z) = position;
        let chunk_coordinates = Map::convert_to_chunk_coordinates(x, y);
        if z == self.position.2 {
            if let Some(chunk) = map.chunks.get(&chunk_coordinates) {
                if let Some(layer) = chunk.layers.get(&z) {
                    if layer.visible_tiles.contains(&(x, y)) {
                        return true;
                    }
                }
            }
        }
        false
    }

    /// create a line between (x0, y0) and (x1, y1)
    fn bresenham_line(x0: i32, y0: i32, x1: i32, y1: i32) -> Vec<(i32, i32)> {
        let mut points = Vec::new();

        let dx = (x1 - x0).abs();
        let dy = -(y1 - y0).abs();
        let mut err = dx + dy;
        let mut x = x0;
        let mut y = y0;
        let sx = if x0 < x1 { 1 } else { -1 };
        let sy = if y0 < y1 { 1 } else { -1 };

        loop {
            points.push((x, y));
            if x == x1 && y == y1 {
                break;
            }
            let e2 = 2 * err;
            if e2 >= dy {
                err += dy;
                x += sx;
            }
            if e2 <= dx {
                err += dx;
                y += sy;
            }
        }
        points
    }

    /// compute all line of sight to finds tiles that are visible by the player
    pub fn in_line_of_sight(&self, start: (i32, i32, i32), target: (i32, i32), map: &Map) -> bool {
        let line = Self::bresenham_line(start.0, start.1, target.0, target.1);
        for (i, &(x, y)) in line.iter().enumerate().skip(1) {
            // if it's the target it means its visible
            if (x, y) == target {
                return true;
            }
            if let Some(tile) = map.get_tile((x, y, self.position.2)) {
                if tile.block_sight {
                    return false;
                }
            } else {
                return false;
            }
        }
        true
    }
}

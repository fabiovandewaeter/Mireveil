use std::collections::HashSet;

use ratatui::{layout::Rect, style::Color};

use crate::{entities::entity::Entity, map::map::Map};

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
pub fn in_line_of_sight(start: (i32, i32), target: (i32, i32), map: &Map) -> bool {
    let line = bresenham_line(start.0, start.1, target.0, target.1);
    for (i, &(x, y)) in line.iter().enumerate().skip(1) {
        // if it's the target it means its visible
        if (x, y) == target {
            return true;
        }
        if let Some(tile) = map.get_tile(x, y, map.visible_layer) {
            if tile.block_sight {
                return false;
            }
        } else {
            return false;
        }
    }
    true
}

fn compute_fov(center: (i32, i32), range: i32, map: &Map) -> HashSet<(i32, i32)> {
    let mut visible = HashSet::new();
    for y in (center.1 - range)..=(center.1 + range) {
        for x in (center.0 - range)..=(center.0 + range) {
            let dx = x - center.0;
            let dy = y - center.1;
            if dx * dx + dy * dy <= range * range {
                if in_line_of_sight(center, (x, y), map) {
                    visible.insert((x, y));
                }
            }
        }
    }
    visible
}

/// update list of visible tiles of the map
pub fn update_visibility(player_pos: (i32, i32), range: i32, map: &mut Map) {
    let visible = compute_fov(player_pos, range, map);
    map.visible_tiles = visible.clone();
    for pos in visible {
        map.revealed_tiles.insert(pos);
    }
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

pub fn calculate_camera_position(player: &Entity, area: Rect) -> (i32, i32) {
    (
        player.position.0 - (area.width as i32 / 2),
        player.position.1 - (area.height as i32 / 2),
    )
}

/// returns true if the entity is visible by the camera, false otherwise
pub fn visible_on_screen(entity: &Entity, area: Rect, camera_position: (i32, i32)) -> bool {
    let screen_x = entity.position.0 - camera_position.0;
    let screen_y = entity.position.1 - camera_position.1;

    screen_x >= 0 && screen_x < area.width as i32 && screen_y >= 0 && screen_y < area.height as i32
}

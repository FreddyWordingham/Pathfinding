use bevy::prelude::*;
use bevy_simple_tilemap::prelude::*;
use ndarray::Array2;
use pathfinding::prelude::*;

use crate::prelude::*;

pub fn set_active_tile_coords_to_start(
    cursor_tile_coords: Res<CursorTileCoords>,
    keyboard_input: Res<ButtonInput<KeyCode>>,
    mut start: ResMut<Start>,
    mut query: Query<&mut TileMap>,
    mut previous_cursor_tile_coords: Local<IVec2>,
) {
    if keyboard_input.pressed(KeyCode::KeyG) {
        let mut tilemap = query.single_mut();

        // Remove the previous graphic
        tilemap.set_tile(
            previous_cursor_tile_coords.extend(MAP_LAYER_START),
            Some(Tile {
                sprite_index: 0,
                ..Default::default()
            }),
        );

        // Show graphic
        tilemap.set_tile(
            cursor_tile_coords.0.extend(MAP_LAYER_START),
            Some(Tile {
                sprite_index: 7,
                color: Color::GREEN,
                ..Default::default()
            }),
        );
        *previous_cursor_tile_coords = cursor_tile_coords.0;

        // Update the Start resource
        start.0 = (
            cursor_tile_coords.0.x as usize,
            cursor_tile_coords.0.y as usize,
        );
    }
}

pub fn set_active_tile_coords_to_end(
    cursor_tile_coords: Res<CursorTileCoords>,
    keyboard_input: Res<ButtonInput<KeyCode>>,
    mut end: ResMut<End>,
    mut query: Query<&mut TileMap>,
    mut previous_cursor_tile_coords: Local<IVec2>,
) {
    if keyboard_input.pressed(KeyCode::KeyH) {
        let mut tilemap = query.single_mut();

        // Remove the previous graphic
        tilemap.set_tile(
            previous_cursor_tile_coords.extend(MAP_LAYER_END),
            Some(Tile {
                sprite_index: 0,
                ..Default::default()
            }),
        );

        // Show graphic
        tilemap.set_tile(
            cursor_tile_coords.0.extend(MAP_LAYER_END),
            Some(Tile {
                sprite_index: 7,
                color: Color::RED,
                ..Default::default()
            }),
        );
        *previous_cursor_tile_coords = cursor_tile_coords.0;

        // Insert the a End resource
        end.0 = (
            cursor_tile_coords.0.x as usize,
            cursor_tile_coords.0.y as usize,
        );
    }
}

pub fn run_pathfinding(
    map: Res<Map>,
    keyboard_input: Res<ButtonInput<KeyCode>>,
    start: ResMut<Start>,
    end: ResMut<End>,
    mut query: Query<&mut TileMap>,
    mut ready_to_run: Local<bool>,
) {
    if keyboard_input.pressed(KeyCode::Space) && *ready_to_run {
        if start.0 == end.0 {
            return;
        }

        // Run A-star algorithm to find the shortest path
        let result = astar(
            &start.0,
            |&pos| neighbors(pos, &map.0),
            |&pos| heuristic(pos, end.0),
            |&pos| pos == end.0,
        );

        // Highlight the path
        match result {
            Some((path, cost)) => {
                println!("Path found with cost {}", cost);
                let mut tilemap = query.single_mut();
                for (x, y) in path {
                    tilemap.set_tile(
                        (x as i32, y as i32, MAP_LAYER_PATH).into(),
                        Some(Tile {
                            sprite_index: 2,
                            color: Color::BLUE,
                            ..Default::default()
                        }),
                    );
                }
            }
            None => {
                println!("No path found");
            }
        }

        *ready_to_run = false;
    } else if keyboard_input.just_released(KeyCode::Space) {
        *ready_to_run = true;
    }
}

// Define the function to convert the grid to a pathfinding-compatible format
fn neighbors(pos: (usize, usize), grid: &Array2<i32>) -> Vec<((usize, usize), i32)> {
    let (x, y) = pos;
    let mut result = Vec::new();

    let directions = [
        (1, 0),  // Right
        (0, 1),  // Down
        (-1, 0), // Left
        (0, -1), // Up
    ];

    for &(dx, dy) in &directions {
        let nx = x as isize + dx;
        let ny = y as isize + dy;
        if nx >= 0 && nx < grid.nrows() as isize && ny >= 0 && ny < grid.ncols() as isize {
            let cost = grid[(nx as usize, ny as usize)] * 10000;
            result.push(((nx as usize, ny as usize), cost));
        }
    }

    result
}

// Define the heuristic function using the Manhattan distance
fn heuristic(a: (usize, usize), b: (usize, usize)) -> i32 {
    ((a.0 as isize - b.0 as isize).abs() + (a.1 as isize - b.1 as isize).abs()) as i32
}

use bevy::{prelude::*, window::PrimaryWindow};
use bevy_simple_tilemap::prelude::*;

use crate::prelude::*;

pub fn update_cursor_tile_coords(
    q_window: Query<&Window, With<PrimaryWindow>>,
    q_camera: Query<(&Camera, &GlobalTransform), With<MainCamera>>,
    mut cursor_tile_coords: ResMut<CursorTileCoords>,
) {
    let (camera, camera_transform) = q_camera.single();
    let window = q_window.single();

    if let Some(tile_coords) = get_cursor_tile_coords(window, camera, camera_transform) {
        cursor_tile_coords.0 = tile_coords;
        cursor_tile_coords.0.x = cursor_tile_coords.0.x.clamp(0, MAP_WIDTH as i32 - 1);
        cursor_tile_coords.0.y = cursor_tile_coords.0.y.clamp(0, MAP_HEIGHT as i32 - 1);
    }
}

pub fn report_cursor_tile_coords(cursor_tile_coords: Res<CursorTileCoords>) {
    println!("Cursor tile coords: {:?}", cursor_tile_coords.0);
}

pub fn highlight_active_tile_coords(
    cursor_tile_coords: Res<CursorTileCoords>,
    mut query: Query<&mut TileMap>,
    mut previous_cursor_tile_coords: Local<IVec2>,
) {
    let mut tilemap = query.single_mut();

    // Remove the highlight from the previous cursor tile
    tilemap.set_tile(
        previous_cursor_tile_coords.extend(MAP_LAYER_HIGHLIGHTING),
        Some(Tile {
            sprite_index: 0,
            ..Default::default()
        }),
    );

    // Highlight the new cursor tile
    tilemap.set_tile(
        cursor_tile_coords.0.extend(MAP_LAYER_HIGHLIGHTING),
        Some(Tile {
            sprite_index: 1,
            ..Default::default()
        }),
    );
    *previous_cursor_tile_coords = cursor_tile_coords.0;
}

pub fn set_active_tile_coords_to_something(
    cursor_tile_coords: Res<CursorTileCoords>,
    keyboard_input: Res<ButtonInput<KeyCode>>,
    mut query: Query<&mut TileMap>,
) {
    if keyboard_input.pressed(KeyCode::KeyX) {
        let mut tilemap = query.single_mut();

        tilemap.set_tile(
            cursor_tile_coords.0.extend(MAP_LAYER_FLOOR),
            Some(Tile {
                sprite_index: 3,
                ..Default::default()
            }),
        );
    } else if keyboard_input.pressed(KeyCode::KeyZ) {
        let mut tilemap = query.single_mut();

        tilemap.set_tile(
            cursor_tile_coords.0.extend(MAP_LAYER_FLOOR),
            Some(Tile {
                sprite_index: 5,
                ..Default::default()
            }),
        );
    }
}

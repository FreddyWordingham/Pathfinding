use bevy::prelude::*;

use super::super::constants::*;
use crate::prelude::*;

/// System to trigger a GenerateMapEvent when a key is pressed.
pub fn trigger_generate_map_event(
    keyboard_input: Res<ButtonInput<KeyCode>>,
    mut generate_map_events: EventWriter<GenerateMapEvent>,
) {
    if keyboard_input.just_pressed(GENERATE_MAP) {
        generate_map_events.send(GenerateMapEvent);
    }
}

/// System tp trigger a RedrawMapEvent when a key is pressed.
pub fn trigger_redraw_map(
    keyboard_input: Res<ButtonInput<KeyCode>>,
    mut event_writer: EventWriter<DrawMapEvent>,
) {
    if keyboard_input.just_pressed(REDRAW_MAP) {
        event_writer.send(DrawMapEvent);
    }
}

/// System to change the WallTileType on the map when the mouse buttons are pressed.
pub fn set_map_wall(
    buttons: Res<ButtonInput<MouseButton>>,
    cursor_tile_coords: Res<CursorTileCoords>,
    mut prev_tile_coords: Local<IVec2>,
    mut set_wall_events: EventWriter<SetMapWallEvent>,
) {
    if let Some(coords) = cursor_tile_coords.0 {
        if buttons.pressed(PLACE_WALL) && coords != *prev_tile_coords {
            set_wall_events.send(SetMapWallEvent {
                position: coords,
                wall_tile_type: WallTileType::Wall,
            });
            *prev_tile_coords = coords;
        }
        if buttons.pressed(REMOVE_WALL) && coords != *prev_tile_coords {
            set_wall_events.send(SetMapWallEvent {
                position: coords,
                wall_tile_type: WallTileType::Empty,
            });
            *prev_tile_coords = coords;
        }
    }
}

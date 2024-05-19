use bevy::prelude::*;

use super::super::constants::*;
use crate::prelude::*;

pub fn mutate_map_walls(
    mut update_map_wall_event: EventWriter<UpdateMapWallEvent>,
    buttons: Res<ButtonInput<MouseButton>>,
    cursor_tile_coords: Res<CursorTileCoords>,
) {
    if buttons.pressed(PLACE_WALL) {
        update_map_wall_event.send(UpdateMapWallEvent {
            position: cursor_tile_coords.0,
            wall_tile_type: WallTileType::Wall,
        });
    }
    if buttons.pressed(REMOVE_WALL) {
        update_map_wall_event.send(UpdateMapWallEvent {
            position: cursor_tile_coords.0,
            wall_tile_type: WallTileType::Empty,
        });
    }
}

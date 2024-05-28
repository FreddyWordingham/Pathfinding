use bevy::prelude::*;
use std::collections::HashSet;

use crate::prelude::*;

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

pub fn update_map_wall(
    mut event_reader: EventReader<SetMapWallEvent>,
    mut map: ResMut<Map>,
    mut event_writer: EventWriter<RedrawWallTileEvent>,
) {
    // Update map tiles, and note which tiles need to be redrawn
    let mut tiles_to_redraw = HashSet::new();
    for SetMapWallEvent {
        position,
        wall_tile_type,
    } in event_reader.read()
    {
        map.set_wall_tile(*position, *wall_tile_type);
        tiles_to_redraw.insert(*position);
        for coord in map.adjacent_coordinates_to_tile(*position) {
            tiles_to_redraw.insert(coord);
        }
    }

    // Update the rendered tilemap
    for position in tiles_to_redraw {
        event_writer.send(RedrawWallTileEvent(position));
    }
}

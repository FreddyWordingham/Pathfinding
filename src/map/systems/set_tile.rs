use bevy::prelude::*;
use std::collections::HashSet;

use crate::prelude::*;

/// System to update the rendered tilemap when a wall on the map is changed.
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

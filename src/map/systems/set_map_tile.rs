use bevy::prelude::*;

use crate::prelude::*;

/// System to update the rendered tilemap when a wall on the map is changed.
pub fn set_map_wall_tile(
    mut event_reader: EventReader<SetMapWallEvent>,
    mut map: ResMut<Map>,
    mut event_writer: EventWriter<DrawWallTileEvent>,
) {
    // Update map tiles, and note which tiles need to be redrawn
    for SetMapWallEvent {
        position,
        wall_tile_type,
    } in event_reader.read()
    {
        if !map.supports_wall(*position) {
            continue;
        }

        map.set_wall_tile(*position, *wall_tile_type);
        event_writer.send(DrawWallTileEvent(*position));
    }
}

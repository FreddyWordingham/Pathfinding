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

/// System to check if the tile will interfere with any of the current paths.
pub fn check_for_colliding_walls(
    mut event_reader: EventReader<SetMapWallEvent>,
    mut query: Query<(Entity, &mut Pathing)>,
) {
    for SetMapWallEvent {
        position,
        wall_tile_type,
    } in event_reader.read()
    {
        // If tile is walkable, don't bother checking for collisions.
        if wall_tile_type.is_walkable() {
            continue;
        }

        // If the tile is not walkable, check if it interferes with any paths.
        for (_entity, mut pathing) in query.iter_mut() {
            let mut trunc = None;
            for (index, path_coord) in pathing.path.iter().enumerate() {
                // If the wall is in the path
                if path_coord == position {
                    // And it is ahead of the entities current position along it
                    if index > pathing.current_step {
                        // Shorten the path
                        trunc = Some(index);
                    }
                }
            }

            if let Some(t) = trunc {
                pathing.path.truncate(t);
            }
        }
    }
}

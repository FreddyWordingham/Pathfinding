use bevy::prelude::*;
use bevy_simple_tilemap::prelude::*;

use crate::map::LAYER_WALLS;

use super::super::{resources::position_to_index, Map, UpdateMapWallEvent};

pub fn update_map_wall(
    mut event_reader: EventReader<UpdateMapWallEvent>,
    mut map: ResMut<Map>,
    mut query: Query<&mut TileMap>,
) {
    let mut tilemap = query.get_single_mut().unwrap();

    let mut tiles = Vec::with_capacity(event_reader.len());
    for event in event_reader.read() {
        map.wall_tiles[position_to_index(event.position)] = event.wall_tile_type.clone();
        tiles.push((
            event.position.extend(LAYER_WALLS),
            Some(Tile {
                sprite_index: map.connected_wall_sprite_index(event.position),
                color: Color::WHITE,
                ..Default::default()
            }),
        ));

        for neighbour in map.get_neighbours(event.position) {
            tiles.push((
                neighbour.extend(LAYER_WALLS),
                Some(Tile {
                    sprite_index: map.connected_wall_sprite_index(neighbour),
                    color: Color::WHITE,
                    ..Default::default()
                }),
            ));
        }
    }
    tilemap.set_tiles(tiles);
}

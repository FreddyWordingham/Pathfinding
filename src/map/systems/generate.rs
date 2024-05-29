use bevy::prelude::*;
use bevy_simple_tilemap::prelude::*;

use crate::prelude::*;

pub fn generate_map(
    mut generate_map_events: EventReader<GenerateMapEvent>,
    mut map: ResMut<Map>,
    mut redraw_map_events: EventWriter<DrawMapEvent>,
    mut query: Query<&mut TileMap>,
) {
    for _ in generate_map_events.read() {
        // Clear the map this way because "tilemap.clear()" is causing crash in bevy_simple_tilemap version 0.14.0
        let width = map.floor_tiles.ncols();
        let height = map.floor_tiles.nrows();
        let mut tiles = Vec::with_capacity(width * height * NUM_LAYERS);
        for xi in 0..width {
            for yi in 0..height {
                for layer in 0..NUM_LAYERS {
                    let position = IVec3::new(xi as i32, yi as i32, layer as i32);
                    let (sprite_index, colour) = (0, Color::WHITE);
                    tiles.push((
                        position,
                        Some(Tile {
                            sprite_index,
                            color: colour,
                            ..Default::default()
                        }),
                    ));
                }
            }
        }
        query.single_mut().set_tiles(tiles);

        // let map_builder = MapBuilder::new_empty_box(7, 7);
        let map_builder = MapBuilder::new_empty_island(17, 17);

        *map = map_builder.build();
        redraw_map_events.send(DrawMapEvent);
    }
}

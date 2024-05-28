use bevy::{
    math::{ivec2, ivec3},
    prelude::*,
};
use bevy_simple_tilemap::prelude::*;

use crate::prelude::*;

pub fn trigger_redraw_map(
    keyboard_input: Res<ButtonInput<KeyCode>>,
    mut event_writer: EventWriter<RedrawMapEvent>,
) {
    if keyboard_input.just_pressed(KeyCode::Space) {
        println!("EVENT: RedrawMap");
        event_writer.send(RedrawMapEvent);
    }
}

pub fn redraw_map(
    event_reader: EventReader<RedrawMapEvent>,
    map: Res<Map>,
    mut query: Query<&mut TileMap>,
) {
    if !event_reader.is_empty() {
        query.single_mut().set_tiles(all_layer_sprites(&map));
    }
}

fn all_layer_sprites(map: &Map) -> Vec<(IVec3, Option<Tile>)> {
    let mut tiles = Vec::with_capacity(map.floor_tiles.len() + map.wall_tiles.len());
    tiles.extend(floor_layer_sprites(&map));
    tiles.extend(wall_layer_sprites(&map));
    tiles
}

fn floor_layer_sprites(map: &Map) -> Vec<(IVec3, Option<Tile>)> {
    let mut tiles = Vec::with_capacity(map.wall_tiles.len());
    for y in 0..map.wall_tiles.nrows() {
        for x in 0..map.wall_tiles.ncols() {
            let (sprite_index, colour) = map.floor_tile_glyph(ivec2(x as i32, y as i32));
            tiles.push((
                ivec3(x as i32, y as i32, LAYER_FLOOR),
                Some(Tile {
                    sprite_index,
                    color: colour,
                    ..Default::default()
                }),
            ));
        }
    }
    tiles
}

fn wall_layer_sprites(map: &Map) -> Vec<(IVec3, Option<Tile>)> {
    let mut tiles = Vec::with_capacity(map.wall_tiles.len());
    for y in 0..map.wall_tiles.nrows() {
        for x in 0..map.wall_tiles.ncols() {
            let (sprite_index, colour) = map.wall_tile_glyph(ivec2(x as i32, y as i32));
            tiles.push((
                ivec3(x as i32, y as i32, LAYER_WALLS),
                Some(Tile {
                    sprite_index,
                    color: colour,
                    ..Default::default()
                }),
            ));
        }
    }
    tiles
}

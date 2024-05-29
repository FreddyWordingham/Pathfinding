use bevy::{
    math::{ivec2, ivec3},
    prelude::*,
};
use bevy_simple_tilemap::prelude::*;

use crate::prelude::*;

pub fn redraw_wall_tiles(
    mut event_reader: EventReader<RedrawWallTileEvent>,
    map: Res<Map>,
    mut query: Query<&mut TileMap>,
) {
    let mut tiles = Vec::with_capacity(event_reader.len());
    for RedrawWallTileEvent(position) in event_reader.read() {
        let (sprite_index, colour) = map.wall_tile_glyph(*position);
        tiles.push((
            position.extend(LAYER_WALLS),
            Some(Tile {
                sprite_index,
                color: colour,
                ..Default::default()
            }),
        ));
    }
    query.single_mut().set_tiles(tiles);
}

pub fn redraw_map(
    mut event_reader: EventReader<RedrawMapEvent>,
    map: Res<Map>,
    mut query: Query<&mut TileMap>,
) {
    for _event in event_reader.read() {
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

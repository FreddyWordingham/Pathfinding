use bevy::{
    math::{ivec2, ivec3},
    prelude::*,
};
use bevy_simple_tilemap::prelude::*;
use std::collections::HashSet;

use super::super::constants::*;
use crate::prelude::*;

pub fn draw_wall_tiles(
    mut event_reader: EventReader<DrawWallTileEvent>,
    map: Res<Map>,
    mut query: Query<&mut TileMap>,
) {
    // Create a set of all wall tiles that need to be redrawn
    let mut tiles_to_redraw = HashSet::new();
    for DrawWallTileEvent(position) in event_reader.read() {
        tiles_to_redraw.insert(*position);
        for coord in map.adjacent_coordinates_to_tile(*position) {
            tiles_to_redraw.insert(coord);
        }
    }

    // Redraw all wall tiles that need to be redrawn
    let tiles = tiles_to_redraw
        .iter()
        .map(|&position| {
            let (sprite_index, colour) = map.wall_tile_glyph(position);
            (
                position.extend(LAYER_WALLS),
                Some(Tile {
                    sprite_index,
                    color: colour,
                    ..Default::default()
                }),
            )
        })
        .collect::<Vec<_>>();
    query.single_mut().set_tiles(tiles);
}

pub fn draw_map(
    mut event_reader: EventReader<DrawMapEvent>,
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
    tiles.extend(marker_layer_sprites(&map));
    tiles
}

fn floor_layer_sprites(map: &Map) -> Vec<(IVec3, Option<Tile>)> {
    let mut tiles = Vec::with_capacity(map.wall_tiles.len());
    for y in 0..map.floor_tiles.nrows() {
        for x in 0..map.floor_tiles.ncols() {
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

fn marker_layer_sprites(map: &Map) -> Vec<(IVec3, Option<Tile>)> {
    let mut tiles = Vec::with_capacity(map.spawn_points.len());
    for spawn_point in map.spawn_points.iter() {
        let sprite_index = GLYPH_SPAWN_POINT;
        let colour = Color::GOLD;

        tiles.push((
            spawn_point.extend(LAYER_MARKERS),
            Some(Tile {
                sprite_index,
                color: colour,
                ..Default::default()
            }),
        ));
    }
    tiles
}

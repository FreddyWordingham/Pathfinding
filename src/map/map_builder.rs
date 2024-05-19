use bevy::prelude::*;
use ndarray::Array2;
use rand::Rng;

use super::{FloorTileType, Map, WallTileType};

/// The map of the game.
/// 2D array of tiles are indexed by [row, column]
#[derive(Resource)]
pub struct MapBuilder {
    pub tile_size: Vec2,
    pub tilemap_scale: f32,
    pub floor_tiles: Array2<FloorTileType>,
    pub wall_tiles: Array2<WallTileType>,
}

impl MapBuilder {
    pub fn new(tile_size: Vec2, tilemap_scale: f32, map_size: IVec2) -> Self {
        debug_assert!(tile_size.x > 0.0);
        debug_assert!(tile_size.y > 0.0);
        debug_assert!(tilemap_scale > 0.0);
        debug_assert!(map_size.x > 0);
        debug_assert!(map_size.y > 0);

        Self {
            tile_size: tile_size,
            tilemap_scale: tilemap_scale,
            floor_tiles: Array2::default((map_size.y as usize, map_size.x as usize)),
            wall_tiles: Array2::default((map_size.y as usize, map_size.x as usize)),
        }
    }

    pub fn new_empty_box(tile_size: Vec2, tilemap_scale: f32, map_size: IVec2) -> Self {
        let mut map_builder = Self::new(tile_size, tilemap_scale, map_size);

        // Add vertical walls
        for y in 0..map_size.y {
            map_builder.wall_tiles[(y as usize, 0)] = WallTileType::Wall;
            map_builder.wall_tiles[(y as usize, map_size.x as usize - 1)] = WallTileType::Wall;
        }

        // Add horizontal walls
        for x in 0..map_size.x {
            map_builder.wall_tiles[(0, x as usize)] = WallTileType::Wall;
            map_builder.wall_tiles[(map_size.y as usize - 1, x as usize)] = WallTileType::Wall;
        }

        map_builder
    }

    pub fn new_random(tile_size: Vec2, tilemap_scale: f32, map_size: IVec2) -> Self {
        let mut map_builder = Self::new(tile_size, tilemap_scale, map_size);

        let mut rng = rand::thread_rng();

        // Add vertical walls
        for y in 0..map_size.y {
            map_builder.wall_tiles[(y as usize, 0)] = WallTileType::Wall;
            map_builder.wall_tiles[(y as usize, map_size.x as usize - 1)] = WallTileType::Wall;
        }

        // Add horizontal walls
        for x in 0..map_size.x {
            map_builder.wall_tiles[(0, x as usize)] = WallTileType::Wall;
            map_builder.wall_tiles[(map_size.y as usize - 1, x as usize)] = WallTileType::Wall;
        }

        // Add random walls
        for y in 1..(map_size.y - 1) {
            for x in 1..(map_size.x - 1) {
                if rng.gen_range(0.0..=1.0) < 0.3 {
                    map_builder.wall_tiles[(y as usize, x as usize)] = WallTileType::Wall;
                }
            }
        }

        map_builder
    }

    pub fn build(self) -> Map {
        Map {
            tile_size: self.tile_size,
            tilemap_scale: self.tilemap_scale,
            floor_tiles: self.floor_tiles,
            wall_tiles: self.wall_tiles,
        }
    }
}

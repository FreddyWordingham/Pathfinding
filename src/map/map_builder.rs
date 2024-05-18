use bevy::prelude::*;
use ndarray::Array2;
use rand::Rng;

use super::{Map, TileType};

/// The map of the game
#[derive(Resource)]
pub struct MapBuilder {
    pub tiles: Array2<TileType>, // 2D array of tiles [row, column]
}

impl MapBuilder {
    pub fn new(size: IVec2) -> Self {
        Self {
            tiles: Array2::default((size.y as usize, size.x as usize)),
        }
    }

    pub fn new_empty_box(size: IVec2) -> Self {
        let mut map_builder = Self::new(size);

        // Add vertical walls
        for y in 0..size.y {
            map_builder.tiles[(y as usize, 0)] = TileType::Wall;
            map_builder.tiles[(y as usize, size.x as usize - 1)] = TileType::Wall;
        }

        // Add horizontal walls
        for x in 0..size.x {
            map_builder.tiles[(0, x as usize)] = TileType::Wall;
            map_builder.tiles[(size.y as usize - 1, x as usize)] = TileType::Wall;
        }

        map_builder
    }

    pub fn new_random(size: IVec2) -> Self {
        let mut map_builder = Self::new(size);

        let mut rng = rand::thread_rng();

        // Add vertical walls
        for y in 0..size.y {
            map_builder.tiles[(y as usize, 0)] = TileType::Wall;
            map_builder.tiles[(y as usize, size.x as usize - 1)] = TileType::Wall;
        }

        // Add horizontal walls
        for x in 0..size.x {
            map_builder.tiles[(0, x as usize)] = TileType::Wall;
            map_builder.tiles[(size.y as usize - 1, x as usize)] = TileType::Wall;
        }

        // Add random walls
        for y in 1..(size.y - 1) {
            for x in 1..(size.x - 1) {
                if rng.gen_range(0.0..=1.0) < 0.3 {
                    map_builder.tiles[(y as usize, x as usize)] = TileType::Wall;
                }
            }
        }

        map_builder
    }

    pub fn build(self) -> Map {
        Map { tiles: self.tiles }
    }
}

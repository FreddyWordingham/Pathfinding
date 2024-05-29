use bevy::{math::ivec2, prelude::*};
use ndarray::Array2;

pub use crate::prelude::*;

pub struct MapBuilder {
    floor_tiles: Array2<FloorTileType>,
    wall_tiles: Array2<WallTileType>,
    spawn_points: Vec<IVec2>,
}

impl MapBuilder {
    pub fn new_empty_box(width: usize, height: usize) -> Self {
        let floor_tiles = Array2::from_elem((height, width), FloorTileType::Grass);
        let mut wall_tiles = Array2::from_elem((height, width), WallTileType::Empty);

        for i in 0..width {
            wall_tiles[(0, i)] = WallTileType::Wall;
            wall_tiles[(height - 1, i)] = WallTileType::Wall;
        }
        for i in 0..height {
            wall_tiles[(i, 0)] = WallTileType::Wall;
            wall_tiles[(i, width - 1)] = WallTileType::Wall;
        }

        let centre = ivec2(width as i32 / 2, height as i32 / 2);

        Self {
            floor_tiles,
            wall_tiles,
            spawn_points: vec![centre],
        }
    }

    pub fn new_empty_island(width: usize, height: usize) -> Self {
        let mut floor_tiles = Array2::from_elem((height, width), FloorTileType::Empty);
        let wall_tiles = Array2::from_elem((height, width), WallTileType::Empty);

        for xi in 0..width {
            for yi in 0..height {
                let r = (xi as f32 - width as f32 * 0.5).hypot(yi as f32 - height as f32 * 0.5);
                if r < width as f32 * 0.5 {
                    floor_tiles[(yi, xi)] = FloorTileType::Grass;
                }
            }
        }

        let centre = ivec2(width as i32 / 2, height as i32 / 2);

        Self {
            floor_tiles,
            wall_tiles,
            spawn_points: vec![centre],
        }
    }

    pub fn build(self) -> Map {
        Map::new(self.floor_tiles, self.wall_tiles, self.spawn_points)
    }
}

use bevy::prelude::*;
use ndarray::Array2;

use crate::prelude::*;

/// The map of the game
#[derive(Resource)]
pub struct Map {
    pub tile_size: Vec2,
    pub tilemap_scale: f32,
    pub floor_tiles: Array2<FloorTileType>,
    pub wall_tiles: Array2<WallTileType>,
}

impl Default for Map {
    fn default() -> Self {
        Self {
            tile_size: Vec2::new(TILE_WIDTH, TILE_HEIGHT),
            tilemap_scale: TILEMAP_SCALE,
            floor_tiles: Array2::from_elem((10, 10), FloorTileType::Grass),
            wall_tiles: Array2::from_elem((10, 10), WallTileType::Empty),
        }
    }
}

impl Map {
    pub fn centre(&self) -> Vec2 {
        let width = self.wall_tiles.ncols() as f32 * self.tile_size.x;
        let height = self.wall_tiles.nrows() as f32 * self.tile_size.y;

        Vec2::new(width, height) * 0.5
    }

    pub fn in_bounds(&self, coords: IVec2) -> bool {
        coords.x >= 0
            && coords.x < self.wall_tiles.ncols() as i32
            && coords.y >= 0
            && coords.y < self.wall_tiles.nrows() as i32
    }

    pub fn floor_tile_sprite_index(&self, position: IVec2) -> (u32, Color) {
        debug_assert!(self.in_bounds(position));

        (1, Color::DARK_GRAY)
    }

    pub fn wall_tile_sprite_index(&self, position: IVec2) -> (u32, Color) {
        debug_assert!(self.in_bounds(position));

        match self.wall_tiles[position_to_index(position)] {
            WallTileType::Empty => (1, Color::DARK_GRAY),
            WallTileType::Wall => (1, Color::WHITE),
            // WallTileType::Wall => (self.connected_wall_sprite_index(position), Color::WHITE),
        }
    }
}

fn position_to_index(position: IVec2) -> (usize, usize) {
    (position.y as usize, position.x as usize)
}

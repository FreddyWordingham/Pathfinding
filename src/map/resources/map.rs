use bevy::{math::ivec2, prelude::*};
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
        let mut wall_tiles = Array2::from_elem((10, 10), WallTileType::Empty);

        for i in 0..10 {
            wall_tiles[(0, i)] = WallTileType::Wall;
            wall_tiles[(9, i)] = WallTileType::Wall;
            wall_tiles[(i, 0)] = WallTileType::Wall;
            wall_tiles[(i, 9)] = WallTileType::Wall;

            wall_tiles[(5, i)] = WallTileType::Wall;
            wall_tiles[(i, 5)] = WallTileType::Wall;
        }

        Self {
            tile_size: Vec2::new(TILE_WIDTH, TILE_HEIGHT),
            tilemap_scale: TILEMAP_SCALE,
            floor_tiles: Array2::from_elem((10, 10), FloorTileType::Grass),
            wall_tiles,
        }
    }
}

impl Map {
    // Access

    pub fn set_wall_tile(&mut self, position: IVec2, tile_type: WallTileType) {
        debug_assert!(self.in_bounds(position));

        self.wall_tiles[position_to_index(position)] = tile_type;
    }

    // Geometry

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

    fn are_adjacent_tiles_walls(&self, position: IVec2) -> [bool; 8] {
        let is_wall = |position: IVec2| -> bool {
            if !self.in_bounds(position) {
                return false;
            }
            self.wall_tiles[position_to_index(position)] == WallTileType::Wall
        };

        [
            is_wall(position + ivec2(0, 1)),
            is_wall(position + ivec2(1, 1)),
            is_wall(position + ivec2(1, 0)),
            is_wall(position + ivec2(1, -1)),
            is_wall(position + ivec2(0, -1)),
            is_wall(position + ivec2(-1, -1)),
            is_wall(position + ivec2(-1, 0)),
            is_wall(position + ivec2(-1, 1)),
        ]
    }

    pub fn adjacent_coordinates_to_tile(&self, position: IVec2) -> Vec<IVec2> {
        #[rustfmt::skip]
        const OFFSETS: [(i32, i32); 8] = [
            (-1, -1), (-1, 0), (-1, 1),
            ( 0, -1),          ( 0, 1),
            ( 1, -1), ( 1, 0), ( 1, 1),
        ];

        let mut positions = Vec::with_capacity(8);

        for &(dx, dy) in &OFFSETS {
            let new_position = position + ivec2(dx, dy);
            if self.in_bounds(new_position) {
                positions.push(new_position);
            }
        }

        positions
    }

    // Rendering

    pub fn floor_tile_glyph(&self, position: IVec2) -> (u32, Color) {
        debug_assert!(self.in_bounds(position));

        match self.floor_tiles[position_to_index(position)] {
            FloorTileType::Empty => (GLYPH_EMPTY, Color::BLACK),
            FloorTileType::Grass => (GLYPH_WALL_ENCLOSED, Color::YELLOW_GREEN),
            FloorTileType::Sand => (GLYPH_WALL_ENCLOSED, Color::GOLD),
            FloorTileType::Stone => (GLYPH_WALL_ENCLOSED, Color::GRAY),
        }
    }

    pub fn wall_tile_glyph(&self, position: IVec2) -> (u32, Color) {
        debug_assert!(self.in_bounds(position));

        match self.wall_tiles[position_to_index(position)] {
            WallTileType::Empty => (GLYPH_EMPTY, Color::BLACK),
            WallTileType::Wall => (self.connected_wall_sprite_glyph(position), Color::WHITE),
        }
    }

    fn connected_wall_sprite_glyph(&self, position: IVec2) -> u32 {
        let tile = &self.wall_tiles[position_to_index(position)];

        debug_assert!(self.in_bounds(position));
        debug_assert!(*tile == WallTileType::Wall);

        let adjacent_walls = self.are_adjacent_tiles_walls(position);
        connection_glyph(adjacent_walls)
    }
}

fn position_to_index(position: IVec2) -> (usize, usize) {
    (position.y as usize, position.x as usize)
}

use bevy::{math::ivec2, prelude::*};
use ndarray::Array2;

use crate::prelude::*;

/// The map of the game
#[derive(Resource)]
pub struct Map {
    pub floor_tiles: Array2<FloorTileType>,
    pub wall_tiles: Array2<WallTileType>,
    pub spawn_points: Vec<IVec2>,
}

impl Default for Map {
    fn default() -> Self {
        let floor_tiles = Array2::from_elem((7, 7), FloorTileType::Grass);
        let mut wall_tiles = Array2::from_elem((7, 7), WallTileType::Empty);

        for i in 0..7 {
            wall_tiles[(3, i)] = WallTileType::Wall;
            wall_tiles[(i, 3)] = WallTileType::Wall;
        }

        Self {
            floor_tiles,
            wall_tiles,
            spawn_points: vec![ivec2(1, 1)],
        }
    }
}

impl Map {
    pub fn new(
        floor_tiles: Array2<FloorTileType>,
        wall_tiles: Array2<WallTileType>,
        spawn_points: Vec<IVec2>,
    ) -> Self {
        let map = Self {
            floor_tiles,
            wall_tiles,
            spawn_points,
        };

        debug_assert!(map.is_valid());

        map
    }

    fn is_valid(&self) -> bool {
        // Check non-empty
        if self.floor_tiles.is_empty() || self.wall_tiles.is_empty() || self.spawn_points.is_empty()
        {
            return false;
        }

        // Check floor and wall tiles have the same dimensions
        if self.floor_tiles.shape() != self.wall_tiles.shape() {
            return false;
        }

        // Check spawn points are in bounds and walkable
        for &spawn_point in &self.spawn_points {
            if !self.in_bounds(spawn_point) {
                return false;
            }
            if !self.is_walkable(spawn_point) {
                return false;
            }
        }

        true
    }

    // Access

    pub fn set_wall_tile(&mut self, position: IVec2, tile_type: WallTileType) {
        debug_assert!(self.in_bounds(position));

        self.wall_tiles[position_to_index(position)] = tile_type;
    }

    // Query

    pub fn supports_wall(&self, position: IVec2) -> bool {
        self.floor_tiles[position_to_index(position)].supports_wall()
            && self
                .spawn_points
                .iter()
                .all(|&spawn_point| spawn_point != position)
    }

    pub fn is_walkable(&self, position: IVec2) -> bool {
        self.floor_tiles[position_to_index(position)].is_walkable()
            && self.wall_tiles[position_to_index(position)].is_walkable()
    }

    // Geometry

    pub fn centre(&self) -> Vec2 {
        let width = self.wall_tiles.ncols() as f32 * TILE_WIDTH;
        let height = self.wall_tiles.nrows() as f32 * TILE_HEIGHT;

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

use bevy::{math::ivec2, prelude::*};
use ndarray::Array2;
use pathfinding::prelude::*;

use super::super::{constants::*, utils::*};
use crate::prelude::*;

/// The map of the game
#[derive(Resource)]
pub struct Map {
    pub floor_tiles: Array2<FloorTileType>,
    pub wall_tiles: Array2<WallTileType>,
    pub spawn_coords: Vec<IVec2>,
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
            spawn_coords: vec![ivec2(1, 1)],
        }
    }
}

impl Map {
    pub fn new(
        floor_tiles: Array2<FloorTileType>,
        wall_tiles: Array2<WallTileType>,
        spawn_coords: Vec<IVec2>,
    ) -> Self {
        let map = Self {
            floor_tiles,
            wall_tiles,
            spawn_coords,
        };

        debug_assert!(map.is_valid());

        map
    }

    fn is_valid(&self) -> bool {
        // Check non-empty
        if self.floor_tiles.is_empty() || self.wall_tiles.is_empty() || self.spawn_coords.is_empty()
        {
            return false;
        }

        // Check floor and wall tiles have the same dimensions
        if self.floor_tiles.shape() != self.wall_tiles.shape() {
            return false;
        }

        // Check spawn points are in bounds and walkable
        for &spawn_point in &self.spawn_coords {
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
                .spawn_coords
                .iter()
                .all(|&spawn_point| spawn_point != position)
    }

    pub fn is_walkable(&self, position: IVec2) -> bool {
        self.floor_tiles[position_to_index(position)].is_walkable()
            && self.wall_tiles[position_to_index(position)].is_walkable()
    }

    pub fn shortest_path(
        &self,
        start_coords: IVec2,
        final_coords: IVec2,
    ) -> Option<(Vec<IVec2>, i32)> {
        debug_assert!(self.in_bounds(start_coords));
        debug_assert!(self.in_bounds(final_coords));
        debug_assert!(start_coords != final_coords);

        // If the final tile is blocked, return no path.
        if !self.is_walkable(final_coords) {
            return None;
        }
        // // If the start or final tile is blocked, return no path.
        // if !self.is_walkable(start_coords) || !self.is_walkable(final_coords) {
        //     return None;
        // }

        let floor_affordability = self.floor_tiles.map(|tile| match tile {
            FloorTileType::Empty => 1000,
            _ => 1,
        });
        let wall_affordability = self.wall_tiles.map(|tile| match tile {
            WallTileType::Empty => 1,
            WallTileType::Wall => 1000,
        });
        let total_affordability = floor_affordability + wall_affordability;

        astar(
            &start_coords,
            |&pos| neighbours(pos, &total_affordability),
            |&pos| heuristic(pos, final_coords),
            |&pos| pos == final_coords,
        )
    }

    // Geometry

    pub fn position_to_coords(&self, position: Vec2) -> IVec2 {
        let x = (position.x / (TILE_WIDTH * TILEMAP_SCALE)) as i32;
        let y = (position.y / (TILE_HEIGHT * TILEMAP_SCALE)) as i32;

        ivec2(x, y)
    }

    pub fn coords_to_position(&self, coords: IVec2) -> Vec2 {
        debug_assert!(self.in_bounds(coords));

        let x = coords.x as f32 * TILE_WIDTH * TILEMAP_SCALE;
        let y = coords.y as f32 * TILE_HEIGHT * TILEMAP_SCALE;

        Vec2::new(x, y)
    }

    pub fn centre(&self) -> Vec2 {
        let width = self.wall_tiles.ncols() as f32 * TILE_WIDTH * TILEMAP_SCALE;
        let height = self.wall_tiles.nrows() as f32 * TILE_HEIGHT * TILEMAP_SCALE;

        Vec2::new(width, height) * 0.5
    }

    pub fn in_bounds(&self, coords: IVec2) -> bool {
        coords.x >= 0
            && coords.x < self.wall_tiles.ncols() as i32
            && coords.y >= 0
            && coords.y < self.wall_tiles.nrows() as i32
    }

    /// Return a list of walkable tiles adjacent to the given position
    pub fn adjacent_walkable_tiles(&self, position: IVec2) -> Vec<IVec2> {
        self.adjacent_coordinates_to_tile(position)
            .into_iter()
            .filter(|&coords| self.in_bounds(coords) && self.is_walkable(coords))
            .collect()
    }

    fn are_neighbour_tiles_walls(&self, position: IVec2) -> [bool; 8] {
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
        const OFFSETS: [(i32, i32); 4] = [
                      (-1, 0),
            ( 0, -1),          ( 0, 1),
                      ( 1, 0),
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

    pub fn neighbour_coordinates_to_tile(&self, position: IVec2) -> Vec<IVec2> {
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

        let neighbour_walls = self.are_neighbour_tiles_walls(position);
        connection_glyph(neighbour_walls)
    }
}

fn position_to_index(position: IVec2) -> (usize, usize) {
    (position.y as usize, position.x as usize)
}

// Define the function to convert the grid to a pathfinding-compatible format
fn neighbours(pos: IVec2, grid: &Array2<i32>) -> Vec<(IVec2, i32)> {
    let mut result = Vec::new();

    let directions = [
        (1, 0),  // Right
        (0, 1),  // Down
        (-1, 0), // Left
        (0, -1), // Up
    ];

    for &(dx, dy) in &directions {
        let nx = pos.x as isize + dx;
        let ny = pos.y as isize + dy;

        if nx >= 0 && nx < grid.ncols() as isize && ny >= 0 && ny < grid.nrows() as isize {
            let cost = grid[position_to_index(pos)];
            result.push((ivec2(nx as i32, ny as i32), cost));
        }
    }

    result
}

fn heuristic(a: IVec2, b: IVec2) -> i32 {
    (((((a.x - b.x).pow(2) + (a.y - b.y).pow(2)) as f32).sqrt()) * 10.0) as i32
}

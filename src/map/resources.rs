use bevy::{math::ivec2, prelude::*};
use ndarray::Array2;

use super::{constants::*, tile_types::TileType};

/// The map of the game
#[derive(Resource)]
pub struct Map {
    pub tiles: Array2<TileType>,
}

impl Default for Map {
    fn default() -> Self {
        Map {
            tiles: Array2::default((1, 1)),
        }
    }
}

impl Map {
    pub fn in_bounds(&self, position: IVec2) -> bool {
        position.x >= 0
            && position.x < self.tiles.ncols() as i32
            && position.y >= 0
            && position.y < self.tiles.nrows() as i32
    }

    pub fn tile_sprite_index(&self, position: IVec2) -> (u32, Color) {
        debug_assert!(self.in_bounds(position));

        match self.tiles[position_to_index(position)] {
            TileType::Void => (GLYPH_VOID, Color::WHITE),
            TileType::Floor => (GLYPH_WALL_ENCLOSED, Color::DARK_GRAY),
            TileType::Wall => (self.connected_wall_sprite_index(position), Color::WHITE),
        }
    }

    pub fn is_wall(&self, position: IVec2) -> bool {
        if !self.in_bounds(position) {
            return false;
        }
        self.tiles[position_to_index(position)] == TileType::Wall
    }

    fn connected_wall_sprite_index(&self, position: IVec2) -> u32 {
        debug_assert!(self.in_bounds(position));
        debug_assert!(self.tiles[position_to_index(position)] == TileType::Wall);

        let nn = self.is_wall(position + ivec2(0, 1));
        let ne = self.is_wall(position + ivec2(1, 1));
        let ee = self.is_wall(position + ivec2(1, 0));
        let se = self.is_wall(position + ivec2(1, -1));
        let ss = self.is_wall(position + ivec2(0, -1));
        let sw = self.is_wall(position + ivec2(-1, -1));
        let ww = self.is_wall(position + ivec2(-1, 0));
        let nw = self.is_wall(position + ivec2(-1, 1));

        match (nn, ne, ee, se, ss, sw, ww, nw) {
            (false, _, false, _, false, _, false, _) => GLYPH_WALL_SINGLE,
            (true, true, true, true, true, true, true, true) => GLYPH_WALL_ENCLOSED,
            (true, false, true, false, true, false, true, false) => GLYPH_WALL_CROSS,

            (true, true, true, false, true, false, true, false) => GLYPH_WALL_CROSS_NORTH_EAST,
            (true, false, true, true, true, false, true, false) => GLYPH_WALL_CROSS_SOUTH_EAST,
            (true, false, true, false, true, true, true, false) => GLYPH_WALL_CROSS_SOUTH_WEST,
            (true, false, true, false, true, false, true, true) => GLYPH_WALL_CROSS_NORTH_WEST,

            (true, _, false, _, false, _, false, _) => GLYPH_WALL_FINGER_SOUTH,
            (false, _, true, _, false, _, false, _) => GLYPH_WALL_FINGER_WEST,
            (false, _, false, _, true, _, false, _) => GLYPH_WALL_FINGER_NORTH,
            (false, _, false, _, false, _, true, _) => GLYPH_WALL_FINGER_EAST,

            (true, true, true, _, false, _, false, _) => GLYPH_WALL_CORNER_FILLED_SOUTH_WEST,
            (false, _, true, true, true, _, false, _) => GLYPH_WALL_CORNER_FILLED_NORTH_WEST,
            (false, _, false, _, true, true, true, _) => GLYPH_WALL_CORNER_FILLED_NORTH_EAST,
            (true, _, false, _, false, _, true, true) => GLYPH_WALL_CORNER_FILLED_SOUTH_EAST,

            (true, false, true, _, false, _, false, _) => GLYPH_WALL_CORNER_OPEN_SOUTH_WEST,
            (false, _, true, false, true, _, false, _) => GLYPH_WALL_CORNER_OPEN_NORTH_WEST,
            (false, _, false, _, true, false, true, _) => GLYPH_WALL_CORNER_OPEN_NORTH_EAST,
            (true, _, false, _, false, _, true, false) => GLYPH_WALL_CORNER_OPEN_SOUTH_EAST,

            (true, false, true, true, true, true, true, true) => GLYPH_WALL_CORNER_INNER_SOUTH_WEST,
            (true, true, true, false, true, true, true, true) => GLYPH_WALL_CORNER_INNER_NORTH_WEST,
            (true, true, true, true, true, false, true, true) => GLYPH_WALL_CORNER_INNER_NORTH_EAST,
            (true, true, true, true, true, true, true, false) => GLYPH_WALL_CORNER_INNER_SOUTH_EAST,

            (true, true, true, false, true, true, true, false) => GLYPH_WALL_DIAGONAL,
            (true, false, true, true, true, false, true, true) => GLYPH_WALL_ANTIDIAGONAL,
            (true, _, false, _, true, _, false, _) => GLYPH_WALL_VERTICAL,
            (false, _, true, _, false, _, true, _) => GLYPH_WALL_HORIZONTAL,

            (true, true, true, _, false, _, true, true) => GLYPH_WALL_FACE_SOUTH,
            (true, true, true, true, true, _, false, _) => GLYPH_WALL_FACE_WEST,
            (false, _, true, true, true, true, true, _) => GLYPH_WALL_FACE_NORTH,
            (true, _, false, _, true, true, true, true) => GLYPH_WALL_FACE_EAST,

            (true, false, true, true, true, true, true, false) => GLYPH_WALL_OUTCROP_SOUTH,
            (true, false, true, false, true, true, true, true) => GLYPH_WALL_OUTCROP_WEST,
            (true, true, true, false, true, false, true, true) => GLYPH_WALL_OUTCROP_NORTH,
            (true, true, true, true, true, false, true, false) => GLYPH_WALL_OUTCROP_EAST,

            (true, false, true, _, false, _, true, false) => GLYPH_T_INTERSECTION_SOUTH,
            (true, false, true, false, true, _, false, _) => GLYPH_T_INTERSECTION_WEST,
            (false, _, true, false, true, false, true, _) => GLYPH_T_INTERSECTION_NORTH,
            (true, _, false, _, true, false, true, false) => GLYPH_T_INTERSECTION_EAST,

            (true, true, true, _, false, _, true, false) => GLYPH_T_INTERSECTION_SOUTH_CLOCKWISE,
            (true, false, true, true, true, _, false, _) => GLYPH_T_INTERSECTION_WEST_CLOCKWISE,
            (false, _, true, false, true, true, true, _) => GLYPH_T_INTERSECTION_NORTH_CLOCKWISE,
            (true, _, false, _, true, false, true, true) => GLYPH_T_INTERSECTION_EAST_CLOCKWISE,

            (true, false, true, _, false, _, true, true) => {
                GLYPH_T_INTERSECTION_SOUTH_ANTICLOCKWISE
            }
            (true, true, true, false, true, _, false, _) => GLYPH_T_INTERSECTION_WEST_ANTICLOCKWISE,
            (false, _, true, true, true, false, true, _) => {
                GLYPH_T_INTERSECTION_NORTH_ANTICLOCKWISE
            }
            (true, _, false, _, true, true, true, false) => GLYPH_T_INTERSECTION_EAST_ANTICLOCKWISE,

            _ => GLYPH_WALL_ENCLOSED,
        }
    }
}

fn position_to_index(position: IVec2) -> (usize, usize) {
    (position.y as usize, position.x as usize)
}

fn index_to_position(index: (usize, usize)) -> IVec2 {
    ivec2(index.1 as i32, index.0 as i32)
}

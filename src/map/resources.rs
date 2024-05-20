use bevy::{math::ivec2, prelude::*};
use ndarray::Array2;
use pathfinding::prelude::*;

use super::{
    constants::*,
    tile_types::{FloorTileType, WallTileType},
};

/// Cursor location on the tilemap
#[derive(Resource, Default)]
pub struct CursorTileCoords(pub Option<IVec2>);

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
        Map {
            tile_size: Vec2::new(TILE_WIDTH, TILE_HEIGHT),
            tilemap_scale: TILEMAP_SCALE,
            floor_tiles: Array2::default((1, 1)),
            wall_tiles: Array2::default((1, 1)),
        }
    }
}

impl Map {
    pub fn width(&self) -> usize {
        self.wall_tiles.ncols()
    }

    pub fn height(&self) -> usize {
        self.wall_tiles.nrows()
    }

    pub fn centre(&self) -> Vec2 {
        Vec2::new(
            self.tile_size.x * self.wall_tiles.ncols() as f32 * self.tilemap_scale * 0.5,
            self.tile_size.y * self.wall_tiles.nrows() as f32 * self.tilemap_scale * 0.5,
        )
    }

    pub fn in_bounds(&self, coords: IVec2) -> bool {
        coords.x >= 0
            && coords.x < self.wall_tiles.ncols() as i32
            && coords.y >= 0
            && coords.y < self.wall_tiles.nrows() as i32
    }

    pub fn is_wall(&self, position: IVec2) -> bool {
        if !self.in_bounds(position) {
            return false;
        }
        self.wall_tiles[position_to_index(position)] == WallTileType::Wall
    }

    pub fn position_to_coords(&self, position: Vec2) -> Option<IVec2> {
        let coords = ivec2(
            (position.x / (self.tile_size.x * self.tilemap_scale)) as i32,
            (position.y / (self.tile_size.y * self.tilemap_scale)) as i32,
        );
        if !self.in_bounds(coords) {
            return None;
        }
        return Some(coords);
    }

    pub fn coords_to_position(&self, coords: IVec2) -> Option<Vec2> {
        if !self.in_bounds(coords) {
            return None;
        }
        return Some(Vec2::new(
            coords.x as f32 * self.tile_size.x * self.tilemap_scale,
            coords.y as f32 * self.tile_size.y * self.tilemap_scale,
        ));
    }

    pub fn get_neighbours(&self, position: IVec2) -> Vec<IVec2> {
        const DIRECTIONS: [IVec2; 8] = [
            IVec2::new(0, 1),   // North
            IVec2::new(1, 1),   // North-East
            IVec2::new(1, 0),   // East
            IVec2::new(1, -1),  // South-East
            IVec2::new(0, -1),  // South
            IVec2::new(-1, -1), // South-West
            IVec2::new(-1, 0),  // West
            IVec2::new(-1, 1),  // North-West
        ];

        DIRECTIONS
            .iter()
            .map(|&dir| position + dir)
            .filter(|&pos| self.in_bounds(pos))
            .collect()
    }

    pub fn floor_tile_sprite_index(&self, position: IVec2) -> (u32, Color) {
        debug_assert!(self.in_bounds(position));

        (GLYPH_WALL_ENCLOSED, Color::DARK_GRAY)
    }

    pub fn wall_tile_sprite_index(&self, position: IVec2) -> (u32, Color) {
        debug_assert!(self.in_bounds(position));

        match self.wall_tiles[position_to_index(position)] {
            WallTileType::Empty => (GLYPH_WALL_ENCLOSED, Color::DARK_GRAY),
            WallTileType::Wall => (self.connected_wall_sprite_index(position), Color::WHITE),
        }
    }

    pub fn connected_wall_sprite_index(&self, position: IVec2) -> u32 {
        debug_assert!(self.in_bounds(position));

        if self.wall_tiles[position_to_index(position)] != WallTileType::Wall {
            return GLYPH_VOID;
        }

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
        }
    }

    pub fn tiles_in_vicinity(&self, position: IVec2, radius: i32) -> Vec<IVec2> {
        let mut tiles = Vec::with_capacity((2 * radius + 1).pow(2) as usize);
        let max_radius = heuristic(ivec2(0, 0), ivec2(radius, 0));
        for y in -radius..=radius {
            for x in -radius..=radius {
                let coords = ivec2(position.x + x, position.y + y);
                if heuristic(position, coords) > max_radius {
                    continue;
                }

                if self.in_bounds(coords)
                    && self.wall_tiles[position_to_index(coords)] == WallTileType::Empty
                {
                    tiles.push(coords);
                }
            }
        }
        tiles.shrink_to_fit();
        tiles
    }

    pub fn get_affordance(&self, position: IVec2) -> i32 {
        match self.wall_tiles[position_to_index(position)] {
            WallTileType::Empty => 1,
            WallTileType::Wall => 100,
        }
    }

    pub fn tiles_in_view(&self, position: IVec2, radius: i32) -> Vec<IVec2> {
        self.tiles_in_vicinity(position, radius)
            .into_iter()
            .filter(|&target| self.line_of_sight(position, target))
            .collect()
    }

    pub fn shortest_path(&self, start: IVec2, target: IVec2) -> Option<(Vec<IVec2>, i32)> {
        let affordability = self.wall_tiles.map(|tile| match tile {
            WallTileType::Empty => 1,
            WallTileType::Wall => 100,
        });
        astar(
            &start,
            |&pos| neighbours(pos, &affordability),
            |&pos| heuristic(pos, target),
            |&pos| pos == target,
        )
    }

    pub fn line_of_sight(&self, start: IVec2, target: IVec2) -> bool {
        let mut line = bresenham_line(start, target).into_iter();
        while let Some(pos) = line.next() {
            if self.is_wall(pos) {
                return false;
            }
        }
        true
    }
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

pub fn position_to_index(position: IVec2) -> (usize, usize) {
    (position.y as usize, position.x as usize)
}

pub fn index_to_position(index: (usize, usize)) -> IVec2 {
    ivec2(index.1 as i32, index.0 as i32)
}

fn bresenham_line(origin: IVec2, target: IVec2) -> Vec<IVec2> {
    let mut points = Vec::new();

    let mut x0 = origin.x;
    let mut y0 = origin.y;
    let x1 = target.x;
    let y1 = target.y;

    let dx = (x1 - x0).abs();
    let dy = -(y1 - y0).abs();
    let sx = if x0 < x1 { 1 } else { -1 };
    let sy = if y0 < y1 { 1 } else { -1 };
    let mut err = dx + dy;

    loop {
        points.push(IVec2::new(x0, y0));
        if x0 == x1 && y0 == y1 {
            break;
        }
        let e2 = 2 * err;
        if e2 >= dy {
            err += dy;
            x0 += sx;
        }
        if e2 <= dx {
            err += dx;
            y0 += sy;
        }
    }

    points
}

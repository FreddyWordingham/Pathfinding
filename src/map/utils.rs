use crate::prelude::*;

pub fn connection_glyph(adjacent_walls: [bool; 8]) -> u32 {
    let nn = adjacent_walls[0];
    let ne = adjacent_walls[1];
    let ee = adjacent_walls[2];
    let se = adjacent_walls[3];
    let ss = adjacent_walls[4];
    let sw = adjacent_walls[5];
    let ww = adjacent_walls[6];
    let nw = adjacent_walls[7];

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

        (true, false, true, _, false, _, true, true) => GLYPH_T_INTERSECTION_SOUTH_ANTICLOCKWISE,
        (true, true, true, false, true, _, false, _) => GLYPH_T_INTERSECTION_WEST_ANTICLOCKWISE,
        (false, _, true, true, true, false, true, _) => GLYPH_T_INTERSECTION_NORTH_ANTICLOCKWISE,
        (true, _, false, _, true, true, true, false) => GLYPH_T_INTERSECTION_EAST_ANTICLOCKWISE,
    }
}

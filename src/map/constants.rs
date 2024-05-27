use bevy::math::{vec2, Vec2};

// Atlas
pub const ATLAS_IMAGE: &str = "textures/atlas.png";
pub const ATLAS_COLUMNS: usize = 10;
pub const ATLAS_ROWS: usize = 13;
pub const ATLAS_PADDING: Option<Vec2> = Some(vec2(1.0, 1.0));
pub const ATLAS_OFFSET: Option<Vec2> = None;

// Map
pub const TILE_WIDTH: f32 = 32.0;
pub const TILE_HEIGHT: f32 = TILE_WIDTH;

// Tilemap
pub const TILEMAP_SCALE: f32 = 1.0;
pub const TILEMAP_CAMERA_Z: f32 = -1.0;

// Layers
pub const LAYER_FLOOR: i32 = 0;
pub const LAYER_WALLS: i32 = 1;

// Glyphs
pub const GLYPH_EMPTY: u32 = 0;

pub const GLYPH_WALL_SINGLE: u32 = 10;
pub const GLYPH_WALL_ENCLOSED: u32 = 20;
pub const GLYPH_WALL_CROSS: u32 = 30;

pub const GLYPH_WALL_CROSS_NORTH_EAST: u32 = 9;
pub const GLYPH_WALL_CROSS_SOUTH_EAST: u32 = 19;
pub const GLYPH_WALL_CROSS_SOUTH_WEST: u32 = 29;
pub const GLYPH_WALL_CROSS_NORTH_WEST: u32 = 39;

pub const GLYPH_WALL_FINGER_SOUTH: u32 = 40;
pub const GLYPH_WALL_FINGER_WEST: u32 = 50;
pub const GLYPH_WALL_FINGER_NORTH: u32 = 60;
pub const GLYPH_WALL_FINGER_EAST: u32 = 70;

pub const GLYPH_WALL_CORNER_FILLED_SOUTH_WEST: u32 = 41;
pub const GLYPH_WALL_CORNER_FILLED_NORTH_WEST: u32 = 51;
pub const GLYPH_WALL_CORNER_FILLED_NORTH_EAST: u32 = 61;
pub const GLYPH_WALL_CORNER_FILLED_SOUTH_EAST: u32 = 71;

pub const GLYPH_WALL_CORNER_OPEN_SOUTH_WEST: u32 = 42;
pub const GLYPH_WALL_CORNER_OPEN_NORTH_WEST: u32 = 52;
pub const GLYPH_WALL_CORNER_OPEN_NORTH_EAST: u32 = 62;
pub const GLYPH_WALL_CORNER_OPEN_SOUTH_EAST: u32 = 72;

pub const GLYPH_WALL_CORNER_INNER_SOUTH_WEST: u32 = 43;
pub const GLYPH_WALL_CORNER_INNER_NORTH_WEST: u32 = 53;
pub const GLYPH_WALL_CORNER_INNER_NORTH_EAST: u32 = 63;
pub const GLYPH_WALL_CORNER_INNER_SOUTH_EAST: u32 = 73;

pub const GLYPH_WALL_DIAGONAL: u32 = 44;
pub const GLYPH_WALL_ANTIDIAGONAL: u32 = 54;
pub const GLYPH_WALL_HORIZONTAL: u32 = 64;
pub const GLYPH_WALL_VERTICAL: u32 = 74;

pub const GLYPH_WALL_FACE_SOUTH: u32 = 45;
pub const GLYPH_WALL_FACE_WEST: u32 = 55;
pub const GLYPH_WALL_FACE_NORTH: u32 = 65;
pub const GLYPH_WALL_FACE_EAST: u32 = 75;

pub const GLYPH_WALL_OUTCROP_SOUTH: u32 = 46;
pub const GLYPH_WALL_OUTCROP_WEST: u32 = 56;
pub const GLYPH_WALL_OUTCROP_NORTH: u32 = 66;
pub const GLYPH_WALL_OUTCROP_EAST: u32 = 76;

pub const GLYPH_T_INTERSECTION_SOUTH: u32 = 47;
pub const GLYPH_T_INTERSECTION_WEST: u32 = 57;
pub const GLYPH_T_INTERSECTION_NORTH: u32 = 67;
pub const GLYPH_T_INTERSECTION_EAST: u32 = 77;

pub const GLYPH_T_INTERSECTION_SOUTH_CLOCKWISE: u32 = 48;
pub const GLYPH_T_INTERSECTION_WEST_CLOCKWISE: u32 = 58;
pub const GLYPH_T_INTERSECTION_NORTH_CLOCKWISE: u32 = 68;
pub const GLYPH_T_INTERSECTION_EAST_CLOCKWISE: u32 = 78;

pub const GLYPH_T_INTERSECTION_SOUTH_ANTICLOCKWISE: u32 = 49;
pub const GLYPH_T_INTERSECTION_WEST_ANTICLOCKWISE: u32 = 59;
pub const GLYPH_T_INTERSECTION_NORTH_ANTICLOCKWISE: u32 = 69;
pub const GLYPH_T_INTERSECTION_EAST_ANTICLOCKWISE: u32 = 79;

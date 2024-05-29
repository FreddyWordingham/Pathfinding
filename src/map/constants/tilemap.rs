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
pub const LAYER_MARKERS: i32 = 2;
pub const NUM_LAYERS: usize = 3;

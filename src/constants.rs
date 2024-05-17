use bevy::{math::vec2, prelude::*};

// Map
pub const TILEMAP_IMAGE: &str = "atlas.png";
pub const TILEMAP_COLUMNS: usize = 4;
pub const TILEMAP_ROWS: usize = 1;
pub const TILEMAP_PADDING: Option<Vec2> = Some(vec2(1.0, 1.0));
pub const TILEMAP_OFFSET: Option<Vec2> = None;
pub const TILE_WIDTH: f32 = 16.0;
pub const TILE_HEIGHT: f32 = TILE_WIDTH;
pub const TILEMAP_SCALE: f32 = 1.0;

pub const MAP_WIDTH: i32 = 8;
pub const MAP_HEIGHT: i32 = 6;
pub const MAP_TILE_COUNT: i32 = MAP_WIDTH * MAP_HEIGHT;

// Camera
pub const CAMERA_MOVE_SPEED: f32 = 100.0;
pub const CAMERA_ZOOM_SPEED: f32 = 2.0;

// Camera controls
pub const CAMERA_PAN_RIGHT: KeyCode = KeyCode::KeyD;
pub const CAMERA_PAN_LEFT: KeyCode = KeyCode::KeyA;
pub const CAMERA_PAN_UP: KeyCode = KeyCode::KeyW;
pub const CAMERA_PAN_DOWN: KeyCode = KeyCode::KeyS;
pub const CAMERA_ZOOM_IN: KeyCode = KeyCode::KeyE;
pub const CAMERA_ZOOM_OUT: KeyCode = KeyCode::KeyQ;

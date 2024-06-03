use bevy::prelude::*;

// Spawn controls
pub const SPAWN_MONSTER: KeyCode = KeyCode::KeyM;

// Map controls
pub const PLACE_WALL: MouseButton = MouseButton::Left;
pub const REMOVE_WALL: MouseButton = MouseButton::Right;

pub const GENERATE_MAP: KeyCode = KeyCode::KeyR;
pub const REDRAW_MAP: KeyCode = KeyCode::KeyP;

// Camera controls
pub const CAMERA_RECENTRE: KeyCode = KeyCode::Space;

pub const CAMERA_PAN_RIGHT: KeyCode = KeyCode::KeyD;
pub const CAMERA_PAN_LEFT: KeyCode = KeyCode::KeyA;
pub const CAMERA_PAN_UP: KeyCode = KeyCode::KeyW;
pub const CAMERA_PAN_DOWN: KeyCode = KeyCode::KeyS;

pub const CAMERA_ZOOM_IN: KeyCode = KeyCode::KeyE;
pub const CAMERA_ZOOM_OUT: KeyCode = KeyCode::KeyQ;

pub const CAMERA_MOVE_SPEED: f32 = 500.0;
pub const CAMERA_ZOOM_SPEED: f32 = 1.0;

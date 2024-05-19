use bevy::prelude::*;

// Camera controls
pub const CAMERA_PAN_RIGHT: KeyCode = KeyCode::KeyD;
pub const CAMERA_PAN_LEFT: KeyCode = KeyCode::KeyA;
pub const CAMERA_PAN_UP: KeyCode = KeyCode::KeyW;
pub const CAMERA_PAN_DOWN: KeyCode = KeyCode::KeyS;

pub const CAMERA_ZOOM_IN: KeyCode = KeyCode::KeyE;
pub const CAMERA_ZOOM_OUT: KeyCode = KeyCode::KeyQ;

pub const CAMERA_MOVE_SPEED: f32 = 500.0;
pub const CAMERA_ZOOM_SPEED: f32 = 1.0;

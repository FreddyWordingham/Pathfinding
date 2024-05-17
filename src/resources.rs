use bevy::prelude::*;
use ndarray::Array2;

/// The map of the game
#[derive(Resource, Default)]
pub struct Map(pub Array2<i32>);

/// Cursor location on the tilemap
#[derive(Resource, Default)]
pub struct CursorTileCoords(pub IVec2);

/// Indicates the start location
#[derive(Resource, Default)]
pub struct Start(pub (usize, usize));

/// Indicates the end location
#[derive(Resource, Default)]
pub struct End(pub (usize, usize));

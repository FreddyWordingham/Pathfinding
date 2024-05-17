use bevy::prelude::*;

/// Cursor location on the tilemap
#[derive(Resource, Default)]
pub struct CursorTileCoords(pub IVec2);

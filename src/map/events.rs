use bevy::prelude::*;

use crate::prelude::*;

#[derive(Event)]
pub struct RedrawMapEvent;

#[derive(Event)]
pub struct RedrawWallTileEvent(pub IVec2);

#[derive(Event)]
pub struct SetMapWallEvent {
    pub position: IVec2,
    pub wall_tile_type: WallTileType,
}

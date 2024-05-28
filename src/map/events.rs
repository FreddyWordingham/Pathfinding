use bevy::prelude::*;

use crate::prelude::*;

#[derive(Event)]
pub struct RedrawMapEvent;

#[derive(Event)]
pub struct UpdateMapWallEvent {
    pub position: IVec2,
    pub wall_tile_type: WallTileType,
}

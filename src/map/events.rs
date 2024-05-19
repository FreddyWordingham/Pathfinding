use bevy::prelude::*;

use super::WallTileType;

#[derive(Event)]
pub struct UpdateMapWallEvent {
    pub position: IVec2,
    pub wall_tile_type: WallTileType,
}

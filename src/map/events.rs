use bevy::prelude::*;

use super::TileType;

#[derive(Event)]
pub struct UpdateMapWallEvent {
    pub position: IVec2,
    pub tile_type: TileType,
}

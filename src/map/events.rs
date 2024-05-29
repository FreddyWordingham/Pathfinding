use bevy::prelude::*;

use crate::prelude::*;

#[derive(Event)]
pub struct GenerateMapEvent;

#[derive(Event)]
pub struct CentreCamera;

#[derive(Event)]
pub struct DrawMapEvent;

#[derive(Event)]
pub struct DrawWallTileEvent(pub IVec2);

#[derive(Event)]
pub struct SetMapWallEvent {
    pub position: IVec2,
    pub wall_tile_type: WallTileType,
}

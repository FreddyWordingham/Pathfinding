use bevy::prelude::*;

use crate::prelude::*;

pub fn get_cursor_tile_coords(
    window: &Window,
    camera: &Camera,
    camera_transform: &GlobalTransform,
) -> Option<IVec2> {
    if let Some(world_position) = window
        .cursor_position()
        .and_then(|cursor| camera.viewport_to_world(camera_transform, cursor))
        .map(|ray| ray.origin.truncate())
    {
        let tile_fx = (world_position.x / TILE_WIDTH) + (TILEMAP_SCALE * 0.5);
        let tile_fy = (world_position.y / TILE_HEIGHT) + (TILEMAP_SCALE * 0.5);

        let tile_ix = tile_fx.floor() as i32;
        let tile_iy = tile_fy.floor() as i32;

        return Some(IVec2::new(tile_ix as i32, tile_iy as i32));
    }
    return None;
}

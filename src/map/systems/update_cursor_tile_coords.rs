use bevy::{prelude::*, window::PrimaryWindow};

use super::super::resources::{CursorTileCoords, Map};

pub fn update_cursor_tile_coords(
    window: Query<&Window, With<PrimaryWindow>>,
    camera: Query<(&Camera, &GlobalTransform)>,
    map: Res<Map>,
    mut cursor_tile_coords: ResMut<CursorTileCoords>,
) {
    let (camera, camera_transform) = camera.single();
    let window = window.single();

    if let Some(tile_coords) = get_cursor_tile_coords(
        window,
        camera,
        camera_transform,
        map.tile_size,
        map.tilemap_scale,
    ) {
        cursor_tile_coords.0 = tile_coords;
        cursor_tile_coords.0.x = cursor_tile_coords.0.x.clamp(0, map.width() as i32 - 1);
        cursor_tile_coords.0.y = cursor_tile_coords.0.y.clamp(0, map.height() as i32 - 1);
    }
}

pub fn get_cursor_tile_coords(
    window: &Window,
    camera: &Camera,
    camera_transform: &GlobalTransform,
    tile_size: Vec2,
    tilemap_scale: f32,
) -> Option<IVec2> {
    if let Some(world_position) = window
        .cursor_position()
        .and_then(|cursor| camera.viewport_to_world(camera_transform, cursor))
        .map(|ray| ray.origin.truncate())
    {
        let tile_fx = (world_position.x / tile_size.x) + (tilemap_scale * 0.5);
        let tile_fy = (world_position.y / tile_size.y) + (tilemap_scale * 0.5);

        let tile_ix = tile_fx.floor() as i32;
        let tile_iy = tile_fy.floor() as i32;

        return Some(IVec2::new(tile_ix as i32, tile_iy as i32));
    }
    return None;
}

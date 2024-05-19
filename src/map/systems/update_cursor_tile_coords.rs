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

    cursor_tile_coords.0 = get_cursor_tile_coords(window, camera, camera_transform, &map);
}

pub fn get_cursor_tile_coords(
    window: &Window,
    camera: &Camera,
    camera_transform: &GlobalTransform,
    map: &Map,
) -> Option<IVec2> {
    if let Some(world_position) = window
        .cursor_position()
        .and_then(|cursor| camera.viewport_to_world(camera_transform, cursor))
        .map(|ray| ray.origin.truncate())
    {
        let tile_fx = (world_position.x / map.tile_size.x) + (map.tilemap_scale * 0.5);
        let tile_fy = (world_position.y / map.tile_size.y) + (map.tilemap_scale * 0.5);

        let tile_ix = tile_fx.floor() as i32;
        let tile_iy = tile_fy.floor() as i32;

        let coords = IVec2::new(tile_ix, tile_iy);

        if !map.in_bounds(coords) {
            return None;
        }
        return Some(coords);
    }
    return None;
}

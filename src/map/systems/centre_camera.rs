use bevy::prelude::*;

use super::super::constants::*;
use crate::prelude::*;

/// Centre the camera on the map.
pub fn centre_camera(
    mut query: Query<&mut Transform, With<Camera2d>>,
    map: Res<Map>,
    mut centre_camera_events: EventReader<CentreCamera>,
) {
    for _ in centre_camera_events.read() {
        let centre = map.centre();
        for mut transform in query.iter_mut() {
            transform.translation = centre.extend(TILEMAP_CAMERA_Z);
        }
    }
}

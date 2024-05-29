use bevy::prelude::*;

use crate::prelude::*;

/// Centre the camera on the map.
pub fn centre_camera(
    mut query: Query<&mut Transform, With<Camera>>,
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

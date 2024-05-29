use bevy::prelude::*;
use bevy_tweening::{lens::TransformPositionLens, Animator, EaseFunction, Tween};
use std::time::Duration;

use super::super::constants::*;
use crate::prelude::*;

/// Centre the camera on the map.
pub fn centre_camera(
    mut commands: Commands,
    mut query: Query<(Entity, &Transform), With<Camera2d>>,
    map: Res<Map>,
    mut centre_camera_events: EventReader<CentreCamera>,
) {
    for _ in centre_camera_events.read() {
        let centre = map.centre();

        for (entity, transform) in query.iter_mut() {
            let tween = Tween::new(
                EaseFunction::CubicInOut,
                Duration::from_millis(1000),
                TransformPositionLens {
                    start: transform.translation,
                    end: centre.extend(TILEMAP_CAMERA_Z),
                },
            );
            commands.entity(entity).insert(Animator::new(tween));
        }
    }
}

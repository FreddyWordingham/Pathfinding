use bevy::prelude::*;
use bevy_tweening::{lens::TransformPositionLens, Animator, EaseFunction, Tween, TweenCompleted};
use std::time::Duration;

use super::super::constants::*;
use crate::prelude::*;

// Tween user data
const CAMERA_PAN_MOVEMENT_COMPLETED: u64 = 42;

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
            )
            .with_completed_event(CAMERA_PAN_MOVEMENT_COMPLETED);
            commands.entity(entity).insert(Animator::new(tween));
        }
    }
}

pub fn clean_up_completed_tweens(mut commands: Commands, mut reader: EventReader<TweenCompleted>) {
    for ev in reader.read() {
        match ev.user_data {
            CAMERA_PAN_MOVEMENT_COMPLETED => {
                commands.entity(ev.entity).remove::<Animator<Transform>>();
            }
            _ => {}
        }
    }
}

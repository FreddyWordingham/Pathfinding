use bevy::prelude::*;
use bevy_tweening::Animator;

use super::super::constants::*;
use crate::prelude::*;

pub fn centre_camera(
    keyboard_input: Res<ButtonInput<KeyCode>>,
    mut recentre_camera_events: EventWriter<CentreCamera>,
) {
    if keyboard_input.pressed(CAMERA_RECENTRE) {
        recentre_camera_events.send(CentreCamera);
    }
}

pub fn camera_movement(
    mut camera_transform_query: Query<
        &mut Transform,
        (With<Camera2d>, Without<Animator<Transform>>),
    >,
    keyboard_input: Res<ButtonInput<KeyCode>>,
    time: Res<Time>,
) {
    if let Some(mut tf) = camera_transform_query.iter_mut().next() {
        if keyboard_input.pressed(CAMERA_ZOOM_IN) {
            tf.scale -= Vec3::splat(CAMERA_ZOOM_SPEED) * time.delta_seconds();
        } else if keyboard_input.pressed(CAMERA_ZOOM_OUT) {
            tf.scale += Vec3::splat(CAMERA_ZOOM_SPEED) * time.delta_seconds();
        }

        if keyboard_input.pressed(CAMERA_PAN_LEFT) {
            tf.translation.x -= CAMERA_MOVE_SPEED * time.delta_seconds();
        } else if keyboard_input.pressed(CAMERA_PAN_RIGHT) {
            tf.translation.x += CAMERA_MOVE_SPEED * time.delta_seconds();
        }

        if keyboard_input.pressed(CAMERA_PAN_DOWN) {
            tf.translation.y -= CAMERA_MOVE_SPEED * time.delta_seconds();
        } else if keyboard_input.pressed(CAMERA_PAN_UP) {
            tf.translation.y += CAMERA_MOVE_SPEED * time.delta_seconds();
        }
    }
}

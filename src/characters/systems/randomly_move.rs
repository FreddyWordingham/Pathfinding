use bevy::prelude::*;
use bevy_rand::prelude::*;
use bevy_tweening::{lens::TransformPositionLens, Animator, EaseFunction, Tween};
use rand::prelude::*;
use std::time::Duration;

use super::super::constants::*;
use crate::prelude::*;

pub fn randomly_move(
    mut commands: Commands,
    mut query: Query<
        (
            Entity,
            &Transform,
            &RandomMovement,
            &mut EntropyComponent<WyRand>,
        ),
        Without<Animator<Transform>>,
    >,
    map: Res<Map>,
) {
    for (entity, transform, _, mut rng) in query.iter_mut() {
        let current_coords = map
            .position_to_coords(transform.translation.truncate())
            .unwrap();

        let mut move_options = Vec::new();
        for i in 0..4 {
            let mut offset = IVec2::ZERO;
            match i {
                0 => offset.x = 1,
                1 => offset.x = -1,
                2 => offset.y = 1,
                3 => offset.y = -1,
                _ => {}
            }
            let new_coords = current_coords + offset;
            if map.in_bounds(new_coords) && !map.is_wall(new_coords) {
                move_options.push(new_coords);
            }
        }

        if move_options.is_empty() {
            continue;
        }

        let new_coords = move_options[rng.gen_range(0..move_options.len())];
        let new_position = map.coords_to_position(new_coords).unwrap();

        let tween = Tween::new(
            EaseFunction::CubicInOut,
            Duration::from_millis(1000),
            TransformPositionLens {
                start: transform.translation,
                end: new_position.extend(CHARACTERS_TRANSLATION_Z),
            },
        )
        .with_completed_event(TWEEN_MOVEMENT_COMPLETED);

        commands.entity(entity).insert(Animator::new(tween));
    }
}

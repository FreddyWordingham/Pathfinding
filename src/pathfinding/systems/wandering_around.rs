use bevy::prelude::*;
use bevy_rand::prelude::*;
use bevy_tweening::{lens::TransformPositionLens, Animator, EaseFunction, Tween, TweenCompleted};
use rand::prelude::*;
use std::time::Duration;

use crate::prelude::*;

const WALKING_MOVEMENT_COMPLETED: u64 = 36;

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
        let current_coords = map.position_to_coords(transform.translation.xy());

        let possible_moves = map.adjacent_walkable_tiles(current_coords);
        if possible_moves.is_empty() {
            continue;
        }

        let new_coords = possible_moves[rng.gen_range(0..possible_moves.len())];
        let new_position = map.coords_to_position(new_coords);

        let tween = Tween::new(
            EaseFunction::CubicInOut,
            Duration::from_millis(1000),
            TransformPositionLens {
                start: transform.translation,
                end: new_position.extend(transform.translation.z),
            },
        )
        .with_completed_event(WALKING_MOVEMENT_COMPLETED);

        commands.entity(entity).insert(Animator::new(tween));
    }
}

pub fn clean_up_completed_tweens(mut commands: Commands, mut reader: EventReader<TweenCompleted>) {
    for ev in reader.read() {
        match ev.user_data {
            WALKING_MOVEMENT_COMPLETED => {
                commands.entity(ev.entity).remove::<Animator<Transform>>();
            }
            _ => {}
        }
    }
}

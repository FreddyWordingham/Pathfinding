use std::time::Duration;

use bevy::{math::ivec2, prelude::*};
use bevy_rand::prelude::*;
use bevy_tweening::{lens::TransformPositionLens, Animator, EaseFunction, Tween, TweenCompleted};
use rand::prelude::*;

use crate::prelude::*;

const PATHING_STEP_MOVEMENT_COMPLETED: u64 = 12;

// Find a new path when the previous one runs out.
pub fn find_path(
    mut query: Query<
        (
            Entity,
            &Transform,
            &mut Pathing,
            &mut EntropyComponent<WyRand>,
        ),
        Without<Animator<Transform>>,
    >,
    map: Res<Map>,
) {
    for (_entity, transform, mut pathing, mut rng) in query.iter_mut() {
        // If the entity has no path, or it has completed it's path, find a new one.
        if pathing.current_step >= pathing.path.len() {
            let current_coords = map.position_to_coords(transform.translation.xy());

            // Attempt to find point to path to.
            let mut final_coords = None;
            for _ in 0..10 {
                let dx = rng.gen_range(-5..5);
                let dy = rng.gen_range(-5..5);

                if dx == 0 && dy == 0 {
                    continue;
                }

                let test_coords = current_coords + ivec2(dx, dy);

                if map.in_bounds(test_coords) && map.is_walkable(test_coords) {
                    final_coords = Some(test_coords);
                }
            }
            if let Some(final_coords) = final_coords {
                let path = map.shortest_path(current_coords, final_coords);

                // Check if path was found.
                if let Some(path) = path {
                    // Check if path is too difficult.
                    if path.1 > 100 {
                        continue;
                    }

                    pathing.path = path.0;
                    pathing.current_step = 0;
                } else {
                    println!("Failed to find path between start and final coords.");
                    continue;
                }
            } else {
                println!("Failed to determine final position to path to.");
                continue;
            }
        }
    }
}

// Take the next step along the path.
pub fn take_next_path_step(
    mut commands: Commands,
    mut query: Query<
        (
            Entity,
            &Transform,
            &mut Pathing,
            &mut EntropyComponent<WyRand>,
        ),
        Without<Animator<Transform>>,
    >,
    map: Res<Map>,
) {
    for (entity, transform, mut pathing, mut _rng) in query.iter_mut() {
        pathing.current_step += 1;
        if pathing.path.len() <= pathing.current_step {
            continue;
        }

        let next_coords = pathing.path[pathing.current_step];
        let next_position = map.coords_to_position(next_coords);

        let tween = Tween::new(
            EaseFunction::CubicInOut,
            Duration::from_millis(1000),
            TransformPositionLens {
                start: transform.translation,
                end: next_position.extend(transform.translation.z),
            },
        )
        .with_completed_event(PATHING_STEP_MOVEMENT_COMPLETED);

        commands.entity(entity).insert(Animator::new(tween));
    }
}

pub fn clean_up_completed_pathing_tweens(
    mut commands: Commands,
    mut reader: EventReader<TweenCompleted>,
) {
    for ev in reader.read() {
        match ev.user_data {
            PATHING_STEP_MOVEMENT_COMPLETED => {
                commands.entity(ev.entity).remove::<Animator<Transform>>();
            }
            _ => {}
        }
    }
}

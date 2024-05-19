use bevy::prelude::*;
use bevy_tweening::{lens::TransformPositionLens, Animator, EaseFunction, Sequence, Tween};
use rand::seq::SliceRandom;
use std::time::Duration;

use super::super::constants::*;
use crate::prelude::*;

pub fn walk_around(
    mut commands: Commands,
    mut query: Query<(Entity, &Transform, &WalkingAround), Without<Animator<Transform>>>,
    map: Res<Map>,
) {
    let mut rng_j = rand::thread_rng();

    for (entity, transform, _) in query.iter_mut() {
        println!("Walking around");

        let current_coords = map
            .position_to_coords(transform.translation.truncate())
            .unwrap();

        let mut tiles_in_view = map.tiles_in_view(current_coords, 7);
        tiles_in_view.shuffle(&mut rng_j);
        let mut path_coords = None;
        let mut path_cost = 1000000;
        for tile_coords in tiles_in_view {
            // The current tile.
            if tile_coords == current_coords {
                continue;
            }

            // Tiles that cannot be moved onto.
            if map.is_wall(tile_coords) {
                continue;
            }

            // Tiles that are walkable.
            if let Some((coords, cost)) = map.shortest_path(current_coords, tile_coords) {
                if cost < 100 {
                    path_coords = Some(coords);
                    path_cost = cost;
                    println!("Path cost: {}", path_cost);
                    break;
                }
            }
        }

        if let Some(coords) = path_coords {
            let cost_delta = (1000.0 * path_cost as f32 / coords.len() as f32) as u64;
            println!("{} {} Walking to {:?}", path_cost, cost_delta, coords);

            let mut tweens = Vec::with_capacity(coords.len() - 1);
            let num_steps = coords.len();

            let coord = coords.first().unwrap();
            let mut tween = Tween::new(
                EaseFunction::CubicInOut,
                Duration::from_millis(cost_delta),
                TransformPositionLens {
                    start: transform.translation,
                    end: map
                        .coords_to_position(*coord)
                        .unwrap()
                        .extend(CHARACTERS_TRANSLATION_Z),
                },
            );
            if num_steps == 1 {
                tween = tween.with_completed_event(TWEEN_MOVEMENT_COMPLETED);
            }
            tweens.push(tween);

            for (i, window) in coords.windows(2).enumerate() {
                if let [coords_a, coords_b] = window {
                    let mut tween = Tween::new(
                        EaseFunction::CubicInOut,
                        Duration::from_millis(cost_delta),
                        TransformPositionLens {
                            start: map
                                .coords_to_position(*coords_a)
                                .unwrap()
                                .extend(CHARACTERS_TRANSLATION_Z),
                            end: map
                                .coords_to_position(*coords_b)
                                .unwrap()
                                .extend(CHARACTERS_TRANSLATION_Z),
                        },
                    );
                    if i == num_steps - 2 {
                        tween = tween.with_completed_event(TWEEN_MOVEMENT_COMPLETED);
                    }
                    tweens.push(tween);
                }
            }

            println!("Walking distance {}", tweens.len());

            commands
                .entity(entity)
                .insert(Animator::new(Sequence::new(tweens)));
        }
    }
}

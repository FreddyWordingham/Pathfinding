use bevy::prelude::*;
use bevy_tweening::{lens::TransformPositionLens, Animator, EaseFunction, Sequence, Tween};
use rand::seq::SliceRandom;
use std::time::Duration;

use super::super::constants::*;
use crate::prelude::*;

pub fn walk_around(
    mut commands: Commands,
    mut query: Query<(Entity, &Transform, &mut WalkingAround), Without<Animator<Transform>>>,
    map: Res<Map>,
) {
    let mut rng_j = rand::thread_rng();
    for (entity, transform, mut walking_around) in query.iter_mut() {
        let current_coords = map
            .position_to_coords(transform.translation.truncate())
            .unwrap();

        // If the character has no path, find a new one.
        if walking_around.remaining_path.is_empty() {
            let mut tiles_in_view = map.tiles_in_view(current_coords, 7);
            tiles_in_view.shuffle(&mut rng_j);
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
                        *walking_around = WalkingAround {
                            remaining_path: coords[1..].to_vec(),
                            remaining_cost: cost,
                        };
                        break;
                    }
                }
            }
        }

        // If the character still has no path, skip this iteration.
        if walking_around.remaining_path.is_empty() {
            continue;
        }

        let next_coords = *walking_around.remaining_path.first().unwrap();

        // If the next position is a wall, generate a new path.
        if map.is_wall(next_coords) {
            let mut tiles_in_view = map.tiles_in_view(current_coords, 7);
            tiles_in_view.shuffle(&mut rng_j);
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
                        *walking_around = WalkingAround {
                            remaining_path: coords[1..].to_vec(),
                            remaining_cost: cost,
                        };
                        break;
                    }
                }
            }
        }

        let affordance = map.get_affordance(next_coords);
        *walking_around = WalkingAround {
            remaining_path: walking_around.remaining_path[1..].to_vec(),
            remaining_cost: walking_around.remaining_cost - affordance,
        };
        println!(
            "{} Walking to {:?} -> {:?}",
            affordance, current_coords, next_coords
        );

        let tween = Tween::new(
            EaseFunction::CubicInOut,
            Duration::from_millis(100 * affordance as u64),
            TransformPositionLens {
                start: map
                    .coords_to_position(current_coords)
                    .unwrap()
                    .extend(CHARACTERS_TRANSLATION_Z),
                end: map
                    .coords_to_position(next_coords)
                    .unwrap()
                    .extend(CHARACTERS_TRANSLATION_Z),
            },
        )
        .with_completed_event(TWEEN_MOVEMENT_COMPLETED);
        commands.entity(entity).insert(Animator::new(tween));
    }
}

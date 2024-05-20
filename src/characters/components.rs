use bevy::prelude::*;

/// Randomly move the entity around the map.
#[derive(Component)]
pub struct RandomMovement;

/// Entity walks around the map in a random fashion.
#[derive(Component)]
pub struct WalkingAround {
    pub remaining_path: Vec<IVec2>,
    pub remaining_cost: i32,
}

use bevy::prelude::*;

/// Randomly move the entity around the map.
#[derive(Component)]
pub struct RandomMovement;

/// Searching for a path.
#[derive(Component)]
pub struct Pathing {
    pub path: Vec<IVec2>,
    pub current_step: usize,
}

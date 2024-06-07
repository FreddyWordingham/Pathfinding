use bevy::prelude::*;
use bevy_rand::prelude::*;

mod components;
mod systems;

pub use components::*;
use systems::*;

pub struct PathfindingPlugin;

impl Plugin for PathfindingPlugin {
    fn build(&self, app: &mut App) {
        let seed: u64 = 234;

        app.add_plugins(EntropyPlugin::<WyRand>::with_seed(seed.to_ne_bytes()))
            .add_systems(Update, randomly_move)
            .add_systems(Update, clean_up_completed_tweens.after(randomly_move))
            .add_systems(Update, find_path)
            .add_systems(Update, take_next_path_step.after(find_path))
            .add_systems(
                Update,
                clean_up_completed_pathing_tweens.after(take_next_path_step),
            );
    }
}

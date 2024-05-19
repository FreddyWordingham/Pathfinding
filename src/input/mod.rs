use bevy::prelude::*;

mod constants;
mod systems;

use systems::*;

pub struct InputPlugin;

impl Plugin for InputPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(Update, (camera_movement, mutate_map_walls));
    }
}

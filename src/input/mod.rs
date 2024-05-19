use bevy::prelude::*;

mod constants;
mod systems;

use systems::camera_movement;

pub struct InputPlugin;

impl Plugin for InputPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(Update, camera_movement);
    }
}

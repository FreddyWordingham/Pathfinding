use bevy::prelude::*;

mod constants;
mod systems;

use systems::*;

pub struct InputPlugin;

impl Plugin for InputPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(Update, camera_movement)
            .add_systems(Update, centre_camera)
            .add_systems(Update, trigger_generate_map_event)
            .add_systems(Update, trigger_redraw_map)
            .add_systems(Update, trigger_spawn_monster_event)
            .add_systems(Update, set_map_wall);
    }
}

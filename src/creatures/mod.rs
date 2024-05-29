use bevy::prelude::*;

mod constants;
mod systems;

pub use constants::*;
pub use systems::*;

pub struct CreaturesPlugin;

impl Plugin for CreaturesPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(Update, camera_movement)
            .add_systems(Update, trigger_generate_map_event)
            .add_systems(Update, trigger_redraw_map)
            .add_systems(Update, set_map_wall);
    }
}

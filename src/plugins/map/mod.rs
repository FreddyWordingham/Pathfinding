use bevy::prelude::*;

mod components;
mod constants;
mod resources;
mod setup;
mod systems;
mod utils;

pub struct MapPlugin;

impl Plugin for MapPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(Startup, setup::setup);
    }
}

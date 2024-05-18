use bevy::{prelude::*, window::WindowResolution};
use bevy_simple_tilemap::prelude::*;

fn main() {
    App::new()
        .add_plugins(DefaultPlugins)
        .add_plugins(bevy::diagnostic::FrameTimeDiagnosticsPlugin::default())
        .add_plugins(bevy::diagnostic::LogDiagnosticsPlugin::default())
        .add_systems(Update, bevy::window::close_on_esc)
        .run();
}

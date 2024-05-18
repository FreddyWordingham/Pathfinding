use bevy::prelude::*;
use bevy_pathfinding::prelude::*;

fn main() {
    App::new()
        .add_plugins(DefaultPlugins)
        .add_plugins(bevy::diagnostic::FrameTimeDiagnosticsPlugin::default())
        .add_plugins(bevy::diagnostic::LogDiagnosticsPlugin::default())
        .add_plugins(MapPlugin)
        .add_systems(Update, bevy::window::close_on_esc)
        .run();
}

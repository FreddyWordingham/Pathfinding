use bevy::prelude::*;
use bevy_pathfinding::prelude::*;

fn main() {
    App::new()
        .add_plugins(DefaultPlugins)
        // .add_plugins(bevy::diagnostic::FrameTimeDiagnosticsPlugin::default())
        // .add_plugins(bevy::diagnostic::LogDiagnosticsPlugin::default())
        .add_plugins(MapPlugin)
        .add_systems(Startup, spawn_camera)
        .add_systems(Update, bevy::window::close_on_esc)
        .run();
}

fn spawn_camera(mut commands: Commands, _map: Res<Map>) {
    commands.spawn(Camera2dBundle {
        transform: Transform::from_translation(Vec3::new(0.0, 0.0, 1.0)),
        ..Default::default()
    });
}

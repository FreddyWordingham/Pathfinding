use bevy::prelude::*;
use bevy_inspector_egui::quick::WorldInspectorPlugin;
use bevy_pathfinding_demo::prelude::*;

fn main() {
    App::new()
        .add_plugins(DefaultPlugins.set(ImagePlugin::default_nearest()))
        .add_plugins(WorldInspectorPlugin::new())
        .add_plugins(MapPlugin)
        .add_plugins(InputPlugin)
        .add_systems(Startup, setup)
        .add_systems(Update, bevy::window::close_on_esc)
        .run();
}

// Setup the initial scene.
fn setup(mut commands: Commands, map: Res<Map>) {
    let map_centre = map.centre();

    // Camera
    commands.spawn((
        Name::new("Main camera"),
        Camera2dBundle {
            transform: Transform::from_translation(map_centre.extend(100.0)),
            ..Default::default()
        },
    ));
}

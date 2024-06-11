use bevy::prelude::*;
use bevy_inspector_egui::quick::WorldInspectorPlugin;
use bevy_pathfinding_demo::prelude::*;
use bevy_tweening::TweeningPlugin;

fn main() {
    App::new()
        .add_plugins((DefaultPlugins.set(ImagePlugin::default_nearest()),))
        .add_plugins(WorldInspectorPlugin::new())
        .add_plugins(TweeningPlugin)
        .add_plugins(MapPlugin)
        .add_plugins(InputPlugin)
        .add_plugins(PathfindingPlugin)
        .add_plugins(LightingPlugin)
        .add_systems(Startup, setup)
        .add_systems(Update, bevy::window::close_on_esc)
        .run();
}

// Setup the initial scene.
fn setup(mut commands: Commands, map: Res<Map>, mut events: EventWriter<CentreCamera>) {
    let map_centre = map.centre();

    // Camera
    commands.spawn((
        Name::new("Main camera"),
        Camera2dBundle {
            transform: Transform::from_translation(map_centre.extend(400.0)),
            ..Default::default()
        },
        bevy_pathfinding_demo::prelude::AmbientLight2D {
            colour: Color::rgb(1.0, 0.0, 1.0),
            brightness: 0.9,
        },
    ));
    // Centre the camera on the map
    events.send(CentreCamera);
}

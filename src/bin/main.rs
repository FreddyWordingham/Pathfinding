use bevy::prelude::*;
use bevy_pathfinding::prelude::*;
use rand::Rng;

fn main() {
    App::new()
        .add_plugins(DefaultPlugins.set(ImagePlugin::default_nearest()))
        // .add_plugins(bevy::diagnostic::FrameTimeDiagnosticsPlugin::default())
        // .add_plugins(bevy::diagnostic::LogDiagnosticsPlugin::default())
        .add_plugins(MapPlugin)
        .add_plugins(InputPlugin)
        .add_systems(Startup, spawn_camera)
        .add_systems(Update, bevy::window::close_on_esc)
        .add_systems(Update, place_random_wall)
        .run();
}

fn spawn_camera(mut commands: Commands, _map: Res<Map>) {
    commands.spawn(Camera2dBundle {
        transform: Transform::from_translation(Vec3::new(0.0, 0.0, 1.0)),
        ..Default::default()
    });
}

fn place_random_wall(mut update_map_wall_event: EventWriter<UpdateMapWallEvent>) {
    let rng = &mut rand::thread_rng();
    let x = rng.gen_range(0..100);
    let y = rng.gen_range(0..100);
    update_map_wall_event.send(UpdateMapWallEvent {
        position: IVec2::new(x, y),
        wall_tile_type: WallTileType::Wall,
    });
}

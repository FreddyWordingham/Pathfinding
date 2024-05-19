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
        // .add_systems(Update, place_random_wall)
        .run();
}

fn spawn_camera(mut commands: Commands, map: Res<Map>) {
    let centre = map.centre();
    commands.spawn(Camera2dBundle {
        transform: Transform::from_translation(centre.extend(100.0)),
        ..Default::default()
    });
}

fn place_random_wall(mut update_map_wall_event: EventWriter<UpdateMapWallEvent>, map: Res<Map>) {
    let ncols = map.wall_tiles.ncols();
    let nrows = map.wall_tiles.nrows();

    let rng = &mut rand::thread_rng();
    let x = rng.gen_range(0..ncols as i32);
    let y = rng.gen_range(0..nrows as i32);
    update_map_wall_event.send(UpdateMapWallEvent {
        position: IVec2::new(x, y),
        wall_tile_type: WallTileType::Wall,
    });
}

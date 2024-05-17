use bevy::{
    math::{ivec3, vec2},
    prelude::*,
    window::{PrimaryWindow, WindowResolution},
};
use bevy_simple_tilemap::prelude::*;

const MAP_WIDTH: i32 = 10;
const MAP_HEIGHT: i32 = 10;

const TILE_WIDTH: f32 = 16.0;
const TILE_HEIGHT: f32 = TILE_WIDTH;
const SCALE: f32 = 1.0;

/// The map of the game
#[derive(Resource, Default)]
struct Map {}

/// Cursor location on the tilemap
#[derive(Resource, Default)]
struct CursorTileCoords(IVec2);

/// Used to help identify the main camera
#[derive(Component)]
struct MainCamera;

fn main() {
    App::new()
        .add_plugins(
            DefaultPlugins
                .set(WindowPlugin {
                    primary_window: Some(Window {
                        resolution: WindowResolution::new(1280.0, 720.0)
                            .with_scale_factor_override(1.0),
                        ..Default::default()
                    }),
                    ..default()
                })
                .set(ImagePlugin::default_nearest()),
        )
        // .add_plugins(bevy::diagnostic::FrameTimeDiagnosticsPlugin::default())
        // .add_plugins(bevy::diagnostic::LogDiagnosticsPlugin::default())
        .add_plugins(SimpleTileMapPlugin)
        .add_systems(Startup, setup)
        .add_systems(Update, input_system)
        .add_systems(Update, change_system)
        .init_resource::<CursorTileCoords>()
        .init_resource::<Map>()
        .add_systems(Update, update_cursor_tile_coords)
        .add_systems(Update, click_tile)
        .add_systems(Update, pathing)
        .add_systems(Update, bevy::window::close_on_esc)
        .run();
}

fn update_cursor_tile_coords(
    mut cursor_tile_coords: ResMut<CursorTileCoords>,
    q_window: Query<&Window, With<PrimaryWindow>>,
    q_camera: Query<(&Camera, &GlobalTransform), With<MainCamera>>,
) {
    let (camera, camera_transform) = q_camera.single();
    let window = q_window.single();

    if let Some(world_position) = window
        .cursor_position()
        .and_then(|cursor| camera.viewport_to_world(camera_transform, cursor))
        .map(|ray| ray.origin.truncate())
    {
        let tile_x = ((world_position.x / TILE_WIDTH) + (SCALE * 0.5)) as i32;
        let tile_y = ((world_position.y / TILE_HEIGHT) + (SCALE * 0.5)) as i32;
        cursor_tile_coords.0 = IVec2::new(tile_x, tile_y);
        // eprintln!("Tile Location: {:?}", cursor_tile_coords.0);
    }
}

fn input_system(
    mut camera_transform_query: Query<&mut Transform, With<Camera2d>>,
    mut tilemap_visible_query: Query<&mut Visibility, With<TileMap>>,
    keyboard_input: Res<ButtonInput<KeyCode>>,
    time: Res<Time>,
) {
    const MOVE_SPEED: f32 = 100.0;
    const ZOOM_SPEED: f32 = 2.0;

    if let Some(mut tf) = camera_transform_query.iter_mut().next() {
        if keyboard_input.pressed(KeyCode::KeyX) {
            tf.scale -= Vec3::splat(ZOOM_SPEED) * time.delta_seconds();
        } else if keyboard_input.pressed(KeyCode::KeyZ) {
            tf.scale += Vec3::splat(ZOOM_SPEED) * time.delta_seconds();
        }

        if keyboard_input.pressed(KeyCode::KeyA) {
            tf.translation.x -= MOVE_SPEED * time.delta_seconds();
        } else if keyboard_input.pressed(KeyCode::KeyD) {
            tf.translation.x += MOVE_SPEED * time.delta_seconds();
        }

        if keyboard_input.pressed(KeyCode::KeyS) {
            tf.translation.y -= MOVE_SPEED * time.delta_seconds();
        } else if keyboard_input.pressed(KeyCode::KeyW) {
            tf.translation.y += MOVE_SPEED * time.delta_seconds();
        }

        if keyboard_input.just_pressed(KeyCode::KeyV) {
            // Toggle visibility
            let mut visibility = tilemap_visible_query.iter_mut().next().unwrap();

            if *visibility == Visibility::Hidden {
                *visibility = Visibility::Visible;
            } else {
                *visibility = Visibility::Hidden;
            }
        }
    }
}

fn click_tile(
    mut cursor_tile_coords: ResMut<CursorTileCoords>,
    mouse_button_input: Res<ButtonInput<MouseButton>>,
    q_window: Query<&Window, With<PrimaryWindow>>,
    q_camera: Query<(&Camera, &GlobalTransform), With<MainCamera>>,
    mut query: Query<&mut TileMap>,
) {
    if mouse_button_input.just_pressed(MouseButton::Left) {
        let (camera, camera_transform) = q_camera.single();
        let window = q_window.single();

        if let Some(world_position) = window
            .cursor_position()
            .and_then(|cursor| camera.viewport_to_world(camera_transform, cursor))
            .map(|ray| ray.origin.truncate())
        {
            // Get the tile location
            let tile_x = ((world_position.x / TILE_WIDTH) + (SCALE * 0.5)) as i32;
            let tile_y = ((world_position.y / TILE_HEIGHT) + (SCALE * 0.5)) as i32;
            cursor_tile_coords.0 = IVec2::new(tile_x, tile_y);
            eprintln!("Tile Location: {:?}", cursor_tile_coords.0);

            // Change the tile type
            let mut tilemap = query.iter_mut().next().unwrap();
            let mut tiles: Vec<(IVec3, Option<Tile>)> = Vec::with_capacity((1 * 1) as usize);
            tiles.push((
                ivec3(cursor_tile_coords.0.x, cursor_tile_coords.0.y, 0),
                Some(Tile {
                    sprite_index: 1,
                    ..Default::default()
                }),
            ));
            tilemap.set_tiles(tiles);
        }
    }

    if mouse_button_input.just_pressed(MouseButton::Right) {
        let (camera, camera_transform) = q_camera.single();
        let window = q_window.single();

        if let Some(world_position) = window
            .cursor_position()
            .and_then(|cursor| camera.viewport_to_world(camera_transform, cursor))
            .map(|ray| ray.origin.truncate())
        {
            // Get the tile location
            let tile_x = ((world_position.x / TILE_WIDTH) + (SCALE * 0.5)) as i32;
            let tile_y = ((world_position.y / TILE_HEIGHT) + (SCALE * 0.5)) as i32;
            cursor_tile_coords.0 = IVec2::new(tile_x, tile_y);
            eprintln!("Tile Location: {:?}", cursor_tile_coords.0);

            // Change the tile type
            let mut tilemap = query.iter_mut().next().unwrap();
            let mut tiles: Vec<(IVec3, Option<Tile>)> = Vec::with_capacity((1 * 1) as usize);
            tiles.push((
                ivec3(cursor_tile_coords.0.x, cursor_tile_coords.0.y, 0),
                Some(Tile {
                    sprite_index: 2,
                    ..Default::default()
                }),
            ));
            tilemap.set_tiles(tiles);
        }
    }
}

fn pathing(
    mouse_button_input: Res<ButtonInput<MouseButton>>,
    q_window: Query<&Window, With<PrimaryWindow>>,
    q_camera: Query<(&Camera, &GlobalTransform), With<MainCamera>>,
    mut query: Query<&mut TileMap>,
    mut start: Local<Option<IVec2>>,
    mut end: Local<Option<IVec2>>,
) {
    if mouse_button_input.just_pressed(MouseButton::Left) {
        let (camera, camera_transform) = q_camera.single();
        let window = q_window.single();

        if let Some(world_position) = window
            .cursor_position()
            .and_then(|cursor| camera.viewport_to_world(camera_transform, cursor))
            .map(|ray| ray.origin.truncate())
        {
            // Get the tile location
            let tile_x = ((world_position.x / TILE_WIDTH) + (SCALE * 0.5)) as i32;
            let tile_y = ((world_position.y / TILE_HEIGHT) + (SCALE * 0.5)) as i32;
            let tile_coords = IVec2::new(tile_x, tile_y);

            // Set the start point
            *start = Some(tile_coords);
        }
    }
    if mouse_button_input.just_pressed(MouseButton::Right) {
        let (camera, camera_transform) = q_camera.single();
        let window = q_window.single();

        if let Some(world_position) = window
            .cursor_position()
            .and_then(|cursor| camera.viewport_to_world(camera_transform, cursor))
            .map(|ray| ray.origin.truncate())
        {
            // Get the tile location
            let tile_x = ((world_position.x / TILE_WIDTH) + (SCALE * 0.5)) as i32;
            let tile_y = ((world_position.y / TILE_HEIGHT) + (SCALE * 0.5)) as i32;
            let tile_coords = IVec2::new(tile_x, tile_y);

            // Set the end point
            *end = Some(tile_coords);
        }
    }

    // If we have a start and end point, calculate the path
    if let Some(start_point) = *start {
        if let Some(end_point) = *end {
            println!("Start: {:?}, End: {:?}", start_point, end_point);
            *start = None;
            *end = None;

            // Calculate path
            let mut tilemap = query.iter_mut().next().unwrap();
            let path = Some(vec![
                IVec2::new(0, 0),
                IVec2::new(1, 0),
                IVec2::new(2, 0),
                IVec2::new(3, 0),
                IVec2::new(4, 0),
                IVec2::new(5, 0),
                IVec2::new(6, 0),
                IVec2::new(7, 0),
                IVec2::new(8, 0),
                IVec2::new(9, 0),
            ]);
            if let Some(path) = path {
                for tile in path {
                    let mut tiles: Vec<(IVec3, Option<Tile>)> =
                        Vec::with_capacity((1 * 1) as usize);
                    tiles.push((
                        ivec3(tile.x, tile.y, 0),
                        Some(Tile {
                            sprite_index: 3,
                            ..Default::default()
                        }),
                    ));
                    tilemap.set_tiles(tiles);
                }
            }
        }
    }
}

fn change_system(mut query: Query<&mut TileMap>, mut counter: Local<u32>) {
    let mut tilemap = query.iter_mut().next().unwrap();

    *counter += 1;
    let mut tiles: Vec<(IVec3, Option<Tile>)> = Vec::with_capacity((1 * 1) as usize);
    tiles.push((
        ivec3(*counter as i32, 0, 0),
        Some(Tile {
            sprite_index: (*counter % 4) as u32,
            ..Default::default()
        }),
    ));

    tilemap.set_tiles(tiles);
}

fn setup(
    asset_server: Res<AssetServer>,
    mut commands: Commands,
    mut texture_atlases: ResMut<Assets<TextureAtlasLayout>>,
) {
    let texture = asset_server.load("textures/tilesheet.png");
    let atlas = TextureAtlasLayout::from_grid(
        vec2(TILE_WIDTH, TILE_HEIGHT),
        4,
        1,
        Some(vec2(1.0, 1.0)),
        None,
    );
    let texture_atlas = texture_atlases.add(atlas);

    let total_tiles = MAP_WIDTH * MAP_HEIGHT;
    let mut tiles = Vec::with_capacity(total_tiles as usize);
    for x in 0..MAP_WIDTH {
        for y in 0..MAP_HEIGHT {
            tiles.push((
                ivec3(x, y, 0),
                Some(Tile {
                    sprite_index: 0,
                    ..Default::default()
                }),
            ));
        }
    }

    let mut tilemap = TileMap::default();
    tilemap.set_tiles(tiles);

    // Set up tilemap
    let tilemap_bundle = TileMapBundle {
        tilemap,
        texture,
        atlas: TextureAtlas {
            layout: texture_atlas,
            ..Default::default()
        },
        transform: Transform {
            scale: Vec3::splat(SCALE),
            translation: Vec3::new(0.0, 0.0, 0.0),
            ..Default::default()
        },
        ..Default::default()
    };

    // Spawn camera
    commands.spawn((Camera2dBundle::default(), MainCamera));

    // Spawn tilemap
    commands.spawn(tilemap_bundle);
}

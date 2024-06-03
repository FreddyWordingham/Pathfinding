use bevy::{
    math::{vec2, vec3},
    prelude::*,
};
use bevy_simple_tilemap::prelude::*;

mod constants;
mod events;
mod map_builder;
mod resources;
mod systems;
mod tiles;
mod utils;

use constants::*;
pub use events::*;
pub use resources::*;
use systems::*;
pub use tiles::*;

pub struct MapPlugin;

impl Plugin for MapPlugin {
    fn build(&self, app: &mut App) {
        app.add_plugins(SimpleTileMapPlugin)
            .init_resource::<Map>()
            .init_resource::<CursorTileCoords>()
            .add_event::<GenerateMapEvent>()
            .add_event::<SpawnMonsterEvent>()
            .add_event::<CentreCamera>()
            .add_event::<DrawMapEvent>()
            .add_event::<DrawWallTileEvent>()
            .add_event::<SetMapWallEvent>()
            .add_systems(Startup, setup)
            .add_systems(Update, generate_map)
            .add_systems(Update, centre_camera.after(generate_map))
            .add_systems(Update, clean_up_completed_tweens.after(centre_camera))
            .add_systems(Update, update_cursor_tile_coords.after(generate_map))
            .add_systems(Update, set_map_wall_tile)
            .add_systems(Update, draw_map)
            .add_systems(Update, draw_wall_tiles)
            .add_systems(Update, spawn_monster);
    }
}

fn setup(
    mut commands: Commands,
    asset_server: Res<AssetServer>,
    mut texture_atlases: ResMut<Assets<TextureAtlasLayout>>,
    mut generate_map_events: EventWriter<GenerateMapEvent>,
    // mut draw_map_events: EventWriter<DrawMapEvent>,
) {
    // Load the texture atlas
    let texture = asset_server.load::<Image>(ATLAS_IMAGE);
    let atlas = TextureAtlasLayout::from_grid(
        vec2(TILE_WIDTH, TILE_HEIGHT),
        ATLAS_COLUMNS,
        ATLAS_ROWS,
        ATLAS_PADDING,
        ATLAS_OFFSET,
    );
    let texture_atlas = texture_atlases.add(atlas);

    // Spawn the tilemap
    let tilemap_bundle = TileMapBundle {
        tilemap: TileMap::default(),
        texture,
        atlas: TextureAtlas {
            layout: texture_atlas,
            ..Default::default()
        },
        transform: Transform {
            scale: Vec3::splat(TILEMAP_SCALE),
            translation: vec3(0.0, 0.0, TILEMAP_CAMERA_Z),
            ..Default::default()
        },
        ..Default::default()
    };
    commands.spawn((Name::new("Tilemap"), tilemap_bundle));

    // Trigger the generate map event
    generate_map_events.send(GenerateMapEvent);
    // // Draw the map
    // draw_map_events.send(DrawMapEvent);
}

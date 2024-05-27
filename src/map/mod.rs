use bevy::{
    math::{ivec2, ivec3, vec2, vec3},
    prelude::*,
};
use bevy_simple_tilemap::prelude::*;

mod constants;
mod events;
mod resources;
mod systems;
mod tiles;

pub use constants::*;
pub use events::*;
pub use resources::*;
pub use systems::*;
pub use tiles::*;

pub struct MapPlugin;

impl Plugin for MapPlugin {
    fn build(&self, app: &mut App) {
        app.add_plugins(SimpleTileMapPlugin)
            .init_resource::<Map>()
            .add_systems(Startup, setup);
    }
}

fn setup(
    asset_server: Res<AssetServer>,
    mut commands: Commands,
    mut texture_atlases: ResMut<Assets<TextureAtlasLayout>>,
    mut map: ResMut<Map>,
) {
    println!("Setting up the map");

    // Load the texture atlas
    let texture = asset_server.load::<Image>(ATLAS_IMAGE);
    let texture_atlas = init_texture_atlas(&mut texture_atlases);

    // Initialise the tile sprites
    let floor_tiles = init_floor_tiles(&map);
    let wall_tiles = init_wall_tiles(&map);

    // Create the tilemap
    let tilemap = create_tilemap(floor_tiles, wall_tiles);
    spawn_tilemap(&mut commands, tilemap, texture, texture_atlas);
}

fn init_texture_atlas(
    texture_atlases: &mut ResMut<Assets<TextureAtlasLayout>>,
) -> Handle<TextureAtlasLayout> {
    let atlas = TextureAtlasLayout::from_grid(
        vec2(TILE_WIDTH, TILE_HEIGHT),
        ATLAS_COLUMNS,
        ATLAS_ROWS,
        ATLAS_PADDING,
        ATLAS_OFFSET,
    );
    texture_atlases.add(atlas)
}

fn init_floor_tiles(map: &Map) -> Vec<(IVec3, Option<Tile>)> {
    let mut tiles = Vec::with_capacity(map.wall_tiles.len());
    for y in 0..map.wall_tiles.nrows() {
        for x in 0..map.wall_tiles.ncols() {
            let (sprite_index, colour) = map.floor_tile_sprite_index(ivec2(x as i32, y as i32));
            tiles.push((
                ivec3(x as i32, y as i32, LAYER_FLOOR),
                Some(Tile {
                    sprite_index,
                    color: colour,
                    ..Default::default()
                }),
            ));
        }
    }
    tiles
}

fn init_wall_tiles(map: &Map) -> Vec<(IVec3, Option<Tile>)> {
    let mut tiles = Vec::with_capacity(map.wall_tiles.len());
    for y in 0..map.wall_tiles.nrows() {
        for x in 0..map.wall_tiles.ncols() {
            let (sprite_index, colour) = map.wall_tile_sprite_index(ivec2(x as i32, y as i32));
            tiles.push((
                ivec3(x as i32, y as i32, LAYER_WALLS),
                Some(Tile {
                    sprite_index,
                    color: colour,
                    ..Default::default()
                }),
            ));
        }
    }
    tiles
}

fn create_tilemap(
    floor_tiles: Vec<(IVec3, Option<Tile>)>,
    wall_tiles: Vec<(IVec3, Option<Tile>)>,
) -> TileMap {
    let mut tilemap = TileMap::default();
    tilemap.set_tiles(floor_tiles);
    tilemap.set_tiles(wall_tiles);
    tilemap
}

fn spawn_tilemap(
    commands: &mut Commands,
    tilemap: TileMap,
    texture: Handle<Image>,
    texture_atlas: Handle<TextureAtlasLayout>,
) {
    let tilemap_bundle = TileMapBundle {
        tilemap,
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
}

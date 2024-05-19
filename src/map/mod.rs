use bevy::{
    math::{ivec2, ivec3, vec2},
    prelude::*,
};
use bevy_simple_tilemap::prelude::*;

mod components;
mod constants;
mod map_builder;
mod resources;
mod systems;
mod tile_types;

use constants::*;
use map_builder::MapBuilder;
pub use resources::Map;
use tile_types::TileType;

pub struct MapPlugin;

impl Plugin for MapPlugin {
    fn build(&self, app: &mut App) {
        app.add_plugins(SimpleTileMapPlugin)
            .init_resource::<Map>()
            .add_systems(Startup, setup)
            .add_systems(Update, say_hello);
    }
}

fn setup(
    asset_server: Res<AssetServer>,
    mut commands: Commands,
    mut texture_atlases: ResMut<Assets<TextureAtlasLayout>>,
    mut map: ResMut<Map>,
) {
    // Initialise the map
    let map_builder = MapBuilder::new_random(ivec2(100, 100));
    *map = map_builder.build();

    // Load the texture atlas
    let texture = load_tilemap_texture(&asset_server);
    let texture_atlas = create_texture_atlas(&mut texture_atlases);
    let tiles = generate_initial_tiles(&map);
    let tilemap = create_tilemap(tiles);
    spawn_tilemap(&mut commands, tilemap, texture, texture_atlas);
}

fn load_tilemap_texture(asset_server: &AssetServer) -> Handle<Image> {
    asset_server.load(format!("{}/{}", "textures", TILEMAP_IMAGE))
}

fn create_texture_atlas(
    texture_atlases: &mut ResMut<Assets<TextureAtlasLayout>>,
) -> Handle<TextureAtlasLayout> {
    let atlas = TextureAtlasLayout::from_grid(
        vec2(TILE_WIDTH, TILE_HEIGHT),
        TILEMAP_COLUMNS,
        TILEMAP_ROWS,
        TILEMAP_PADDING,
        TILEMAP_OFFSET,
    );
    texture_atlases.add(atlas)
}

fn generate_initial_tiles(map: &Map) -> Vec<(IVec3, Option<Tile>)> {
    let mut tiles = Vec::with_capacity(map.tiles.len());
    for y in 0..map.tiles.nrows() {
        for x in 0..map.tiles.ncols() {
            let (sprite_index, colour) = map.tile_sprite_index(IVec2::new(x as i32, y as i32));
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

fn create_tilemap(tiles: Vec<(IVec3, Option<Tile>)>) -> TileMap {
    let mut tilemap = TileMap::default();
    tilemap.set_tiles(tiles);
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
            translation: Vec3::new(0.0, 0.0, 0.0),
            ..Default::default()
        },
        ..Default::default()
    };
    commands.spawn(tilemap_bundle);
}

fn say_hello() {
    // println!("Hello, world!");
}

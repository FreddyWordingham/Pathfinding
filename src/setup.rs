use bevy::{
    math::{ivec3, vec2},
    prelude::*,
};
use bevy_simple_tilemap::prelude::*;
use ndarray::Array2;
use rand::random;

use crate::prelude::*;

pub fn setup(
    asset_server: Res<AssetServer>,
    mut commands: Commands,
    mut texture_atlases: ResMut<Assets<TextureAtlasLayout>>,
    mut map: ResMut<Map>,
) {
    *map = Map(generate_map());
    spawn_camera(&mut commands);
    let texture = load_tilemap_texture(&asset_server);
    let texture_atlas = create_texture_atlas(&mut texture_atlases);
    let tiles = generate_initial_tiles(&map);
    let tilemap = create_tilemap(tiles);
    spawn_tilemap(&mut commands, tilemap, texture, texture_atlas);
}

fn generate_map() -> Array2<i32> {
    let mut map = Array2::from_elem([MAP_WIDTH as usize, MAP_HEIGHT as usize], 0);
    for _ in 0..1000 {
        let x = random::<usize>() % MAP_WIDTH as usize;
        let y = random::<usize>() % MAP_HEIGHT as usize;
        map[[x, y]] = 1;
    }
    map
}

fn spawn_camera(commands: &mut Commands) {
    // Spawn camera at center of map
    commands.spawn((
        Camera2dBundle {
            transform: Transform::from_translation(Vec3::new(
                (MAP_WIDTH as f32 * TILE_WIDTH) / 2.0,
                (MAP_HEIGHT as f32 * TILE_HEIGHT) / 2.0,
                1.0,
            )),
            ..Default::default()
        },
        MainCamera,
    ));
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
    let mut tiles = Vec::with_capacity(MAP_TILE_COUNT as usize);
    for x in 0..MAP_WIDTH {
        for y in 0..MAP_HEIGHT {
            let index = match map.0[[x as usize, y as usize]] {
                0 => 5,
                _ => 3,
            };

            tiles.push((
                ivec3(x, y, MAP_LAYER_FLOOR),
                Some(Tile {
                    sprite_index: index,
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

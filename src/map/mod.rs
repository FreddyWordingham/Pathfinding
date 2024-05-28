use bevy::{
    math::{vec2, vec3},
    prelude::*,
};
use bevy_simple_tilemap::prelude::*;

mod constants;
mod events;
mod resources;
mod systems;
mod tiles;
mod utils;

pub use constants::*;
pub use events::*;
pub use resources::*;
pub use systems::*;
pub use tiles::*;
pub use utils::*;

pub struct MapPlugin;

impl Plugin for MapPlugin {
    fn build(&self, app: &mut App) {
        app.add_plugins(SimpleTileMapPlugin)
            .init_resource::<Map>()
            .init_resource::<CursorTileCoords>()
            .add_event::<RedrawMapEvent>()
            .add_systems(Startup, setup)
            .add_systems(Update, update_cursor_tile_coords)
            .add_systems(Update, trigger_redraw_map)
            .add_systems(Update, redraw_map.after(trigger_redraw_map));
    }
}

fn setup(
    mut commands: Commands,
    asset_server: Res<AssetServer>,
    mut texture_atlases: ResMut<Assets<TextureAtlasLayout>>,
    mut redraw_map_events: EventWriter<RedrawMapEvent>,
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

    // Trigger the redraw map event
    redraw_map_events.send(RedrawMapEvent);
}

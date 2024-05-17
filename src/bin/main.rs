use bevy::{prelude::*, window::WindowResolution};
use bevy_simple_tilemap::prelude::*;
use simrpg::prelude::*;

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
        .init_resource::<CursorTileCoords>()
        .init_resource::<Map>()
        .add_systems(Startup, setup)
        .add_systems(Update, bevy::window::close_on_esc)
        .add_systems(Update, (camera_movement, update_cursor_tile_coords))
        .add_systems(
            Update,
            (
                report_cursor_tile_coords,
                highlight_active_tile_coords,
                set_active_tile_coords_to_something,
            )
                .after(update_cursor_tile_coords),
        )
        .run();
}

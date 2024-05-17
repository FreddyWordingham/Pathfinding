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
        .add_systems(Startup, setup)
        .add_systems(Update, bevy::window::close_on_esc)
        .add_systems(Update, camera_movement)
        .run();
}

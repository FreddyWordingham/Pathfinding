use bevy::prelude::*;

fn main() {
    App::new()
        .add_plugins((DefaultPlugins,))
        .add_systems(Startup, setup)
        .add_systems(Update, bevy::window::close_on_esc)
        .run();
}

// Setup the initial scene.
fn setup(mut commands: Commands) {
    // Camera
    commands.spawn(Camera2dBundle::default());
}

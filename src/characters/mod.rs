use bevy::{math::ivec2, prelude::*};
use bevy_rand::prelude::*;
use bevy_tweening::TweeningPlugin;

use crate::prelude::*;

mod components;
mod constants;
mod systems;

pub use components::*;
use constants::*;
use systems::*;

pub struct CharactersPlugin;

impl Plugin for CharactersPlugin {
    fn build(&self, app: &mut App) {
        let seed: u64 = 234;

        app.add_plugins(TweeningPlugin)
            .add_plugins(EntropyPlugin::<WyRand>::with_seed(seed.to_ne_bytes()))
            .add_systems(Startup, spawn_character)
            .add_systems(
                Update,
                (randomly_move, walk_around, clean_up_completed_tweens),
            );
    }
}

fn spawn_character(
    mut commands: Commands,
    asset_server: Res<AssetServer>,
    map: Res<Map>,
    mut rng: ResMut<GlobalEntropy<WyRand>>,
) {
    let xi = (map.width() / 2) as i32;
    let yi = (map.height() / 2) as i32;
    let centre = map.coords_to_position(ivec2(xi, yi)).unwrap();

    for _ in 0..1 {
        commands.spawn((
            Name::new("Spider"),
            SpriteBundle {
                texture: asset_server.load(CHARACTER_IMAGE),
                transform: Transform::from_translation(centre.extend(CHARACTERS_TRANSLATION_Z)),
                ..default()
            },
            // RandomMovement {},
            WalkingAround {
                remaining_path: Vec::new(),
                remaining_cost: 0,
            },
            rng.fork_rng(),
        ));
    }
}

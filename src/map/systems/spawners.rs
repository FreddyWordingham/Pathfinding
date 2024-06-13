use bevy::prelude::*;
use bevy_rand::prelude::*;

use crate::prelude::*;

const MONSTER_Z: f32 = 10.0;
const MONSTER_IMAGE: &str = "characters/monster_spider.png";

/// Spawn a monster on the map.
pub fn spawn_monster(
    mut commands: Commands,
    mut event_reader: EventReader<SpawnMonsterEvent>,
    asset_server: Res<AssetServer>,
    map: ResMut<Map>,
    mut last_spawn_index: Local<usize>,
    mut rng: ResMut<GlobalEntropy<WyRand>>,
) {
    for _ in event_reader.read() {
        for _ in 0..map.spawn_coords.len() {
            let spawn_coord = map.spawn_coords[*last_spawn_index % map.spawn_coords.len()];

            if !map.is_walkable(spawn_coord) {
                continue;
            }
            let spawn_position = map.coords_to_position(spawn_coord);

            commands.spawn((
                Name::new("Monster"),
                SpriteBundle {
                    texture: asset_server.load(MONSTER_IMAGE),
                    transform: Transform::from_translation(spawn_position.extend(MONSTER_Z)),
                    ..Default::default()
                },
                Pathing {
                    path: Vec::new(),
                    current_step: 0,
                },
                rng.fork_rng(),
                Worldly,
                PointLight2D {
                    colour: Color::WHITE,
                    brightness: 0.5,
                    radius: 200.0,
                    falloff: 1.0,
                },
            ));

            *last_spawn_index += 1;
        }
    }
}

use bevy::prelude::*;

use super::super::constants::*;
use crate::prelude::*;

/// System to trigger a SpawnMonsterEvent when a key is pressed.
pub fn trigger_spawn_monster_event(
    keyboard_input: Res<ButtonInput<KeyCode>>,
    mut spawn_monster_events: EventWriter<SpawnMonsterEvent>,
) {
    if keyboard_input.just_pressed(SPAWN_MONSTER) {
        spawn_monster_events.send(SpawnMonsterEvent);
    }
}

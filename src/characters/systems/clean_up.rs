use bevy::prelude::*;
use bevy_tweening::{Animator, TweenCompleted};

use super::super::constants::*;

pub fn clean_up_completed_tweens(mut commands: Commands, mut reader: EventReader<TweenCompleted>) {
    for ev in reader.read() {
        match ev.user_data {
            TWEEN_MOVEMENT_COMPLETED => {
                commands.entity(ev.entity).remove::<Animator<Transform>>();
                println!("Movement completed");
            }
            _ => {
                println!("Unknown user data: {}", ev.user_data);
            }
        }
    }
}

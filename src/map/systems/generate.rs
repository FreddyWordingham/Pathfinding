use bevy::prelude::*;

use crate::prelude::*;

pub fn generate_map(mut redraw_map_events: EventWriter<RedrawMapEvent>) {
    redraw_map_events.send(RedrawMapEvent);
}

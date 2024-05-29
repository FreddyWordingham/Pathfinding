use bevy::prelude::*;

use crate::prelude::*;

pub fn generate_map(
    mut generate_map_events: EventReader<GenerateMapEvent>,
    mut map: ResMut<Map>,
    mut redraw_map_events: EventWriter<RedrawMapEvent>,
) {
    for _ in generate_map_events.read() {
        *map = Map::default();
        redraw_map_events.send(RedrawMapEvent);
    }
}

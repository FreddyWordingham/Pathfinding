use bevy::prelude::*;

use crate::prelude::*;

pub fn generate_map(
    mut generate_map_events: EventReader<GenerateMapEvent>,
    mut map: ResMut<Map>,
    mut redraw_map_events: EventWriter<DrawMapEvent>,
) {
    for _ in generate_map_events.read() {
        // let map_builder = MapBuilder::new_empty_box(7, 7);
        let map_builder = MapBuilder::new_empty_island(17, 17);

        *map = map_builder.build();
        redraw_map_events.send(DrawMapEvent);
    }
}

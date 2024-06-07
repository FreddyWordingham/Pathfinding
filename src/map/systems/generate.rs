use bevy::prelude::*;
use bevy_simple_tilemap::prelude::*;

use super::super::map_builder::*;

pub fn generate_map(
    mut commands: Commands,
    mut generate_map_events: EventReader<GenerateMapEvent>,
    mut map: ResMut<Map>,
    mut redraw_map_events: EventWriter<DrawMapEvent>,
    mut centre_camera_events: EventWriter<CentreCamera>,
    mut worldly_entities: Query<(Entity, &Worldly)>,
    mut query: Query<&mut TileMap>,
) {
    for _ in generate_map_events.read() {
        // Clear the previous map
        query.single_mut().clear();

        // Despawn all Worldly entities
        for (entity, _) in worldly_entities.iter_mut() {
            commands.entity(entity).despawn();
        }

        // let map_builder = MapBuilder::new_empty_box(7, 7);
        let map_builder = MapBuilder::new_empty_island(17, 17);
        *map = map_builder.build();

        // Draw the new map, and re-centre the camera
        redraw_map_events.send(DrawMapEvent);
        centre_camera_events.send(CentreCamera);
    }
}

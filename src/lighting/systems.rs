use bevy::{
    prelude::*,
    render::{
        renderer::{RenderDevice, RenderQueue},
        Extract,
    },
};

use super::{components::*, render::*};

pub fn extract_ambient_lights(
    mut commands: Commands,
    ambient_light_query: Extract<Query<(Entity, &AmbientLight2D)>>,
) {
    for (entity, ambient_light) in &ambient_light_query {
        commands
            .get_or_spawn(entity)
            .insert(ExtractedAmbientLight2D {
                colour: ambient_light.colour.rgba_linear_to_vec4() * ambient_light.brightness,
            });
    }
}

pub fn extract_point_lights(
    mut commands: Commands,
    point_light_query: Extract<Query<(Entity, &PointLight2D, &GlobalTransform, &ViewVisibility)>>,
) {
    for (entity, point_light, global_transform, view_visibility) in &point_light_query {
        if !view_visibility.get() {
            continue;
        }
        commands.get_or_spawn(entity).insert(ExtractedPointLight2D {
            colour: point_light.colour.rgba_linear_to_vec4(),
            transform: *global_transform,
            radius: point_light.radius,
            intensity: point_light.brightness,
            falloff: point_light.falloff,
        });
    }
}

pub fn extract_circular_occluders(
    mut commands: Commands,
    circular_occluder_query: Extract<
        Query<(
            Entity,
            &CircularOccluder2D,
            &GlobalTransform,
            &ViewVisibility,
        )>,
    >,
) {
    for (entity, circular_occluder, global_transform, view_visibility) in &circular_occluder_query {
        if !view_visibility.get() {
            continue;
        }
        commands
            .get_or_spawn(entity)
            .insert(ExtractedCircularOccluder2D {
                transform: *global_transform,
                radius: circular_occluder.radius,
            });
    }
}

pub fn prepare_point_lights(
    render_device: Res<RenderDevice>,
    render_queue: Res<RenderQueue>,
    point_light_query: Query<&ExtractedPointLight2D>,
    mut gpu_point_lights: ResMut<GpuPointLights2DBuffer>,
) {
    let point_light_buffer = gpu_point_lights.buffer.get_mut();

    // Resources are global state, so we need to clear the data from the previous frame.
    point_light_buffer.clear();

    for point_light in &point_light_query {
        point_light_buffer.push(GpuPointLight2D {
            center: point_light.transform.translation().xy(),
            radius: point_light.radius,
            colour: point_light.colour,
            intensity: point_light.intensity,
            falloff: point_light.falloff,
        });
    }

    gpu_point_lights
        .buffer
        .write_buffer(&render_device, &render_queue);
}

pub fn prepare_circle_occluders(
    render_device: Res<RenderDevice>,
    render_queue: Res<RenderQueue>,
    point_light_query: Query<&ExtractedCircularOccluder2D>,
    mut gpu_circular_occluders: ResMut<GpuCircularOccluders2DBuffer>,
) {
    let circular_occluder_buffer = gpu_circular_occluders.buffer.get_mut();

    // Resources are global state, so we need to clear the data from the previous frame.
    circular_occluder_buffer.clear();

    for point_light in &point_light_query {
        circular_occluder_buffer.push(GpuCircularOccluder2D {
            center: point_light.transform.translation().xy(),
            radius: point_light.radius,
        });
    }

    gpu_circular_occluders
        .buffer
        .write_buffer(&render_device, &render_queue);
}

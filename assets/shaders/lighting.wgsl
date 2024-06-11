#import bevy_core_pipeline::fullscreen_vertex_shader::FullscreenVertexOutput
#import bevy_render::view::View


struct AmbientLight {
    colour: vec3<f32>
}

struct PointLight {
    center: vec2f,
    radius: f32,
    colour: vec4<f32>,
    intensity: f32,
    falloff: f32
}

struct CircularOccluder {
    center: vec2f,
    radius: f32,
}


@group(0) @binding(0)
var screen_texture: texture_2d<f32>;

@group(0) @binding(1)
var texture_sampler: sampler;

@group(0) @binding(2)
var<uniform> view: View;

@group(0) @binding(3)
var<uniform> ambient_light: AmbientLight;

@group(0) @binding(4)
var<storage> point_lights: array<PointLight>;

@group(0) @binding(5)
var<storage> circular_occluders: array<CircularOccluder>;


@fragment
fn fragment(vertex: FullscreenVertexOutput) -> @location(0) vec4<f32> {
    let base_colour = textureSample(screen_texture, texture_sampler, vertex.uv);
    return base_colour * vec4(ambient_light.colour, 1.0);
}
#import bevy_core_pipeline::fullscreen_vertex_shader::FullscreenVertexOutput
#import bevy_render::view::View


@group(0) @binding(0)
var screen_texture: texture_2d<f32>;

@group(0) @binding(1)
var texture_sampler: sampler;

@group(0) @binding(2)
var<uniform> view: View;


@fragment
fn fragment(vertex: FullscreenVertexOutput) -> @location(0) vec4<f32> {
    var colour = textureSample(screen_texture, texture_sampler, vertex.uv);
    colour.g = 0.0;
    return colour;
}
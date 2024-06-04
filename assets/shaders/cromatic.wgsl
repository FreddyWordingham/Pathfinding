#import bevy_core_pipeline::fullscreen_vertex_shader::FullscreenVertexOutput
#import bevy_pbr::{
    mesh_view_bindings::globals,
    prepass_utils,
    forward_io::VertexOutput,
}

@group(0) @binding(0) var screen_texture: texture_2d<f32>;
@group(0) @binding(1) var texture_sampler: sampler;
struct PostProcessSettings {
    intensity: f32
}
@group(0) @binding(2) var<uniform> settings: PostProcessSettings;

@fragment
fn fragment(in: FullscreenVertexOutput) -> @location(0) vec4<f32> {
    // // Chromatic aberration strength
    // let offset_strength = settings.intensity;

    // // // Sample each color channel with an arbitrary shift
    // // return vec4<f32>(
    // //     textureSample(screen_texture, texture_sampler, in.uv + vec2<f32>(offset_strength, -offset_strength)).r,
    // //     textureSample(screen_texture, texture_sampler, in.uv + vec2<f32>(-offset_strength, 0.0)).g,
    // //     textureSample(screen_texture, texture_sampler, in.uv + vec2<f32>(0.0, offset_strength)).b,
    // //     1.0
    // // );

    // let uv = in.uv;
    // let base_colour = textureSample(screen_texture, texture_sampler, in.uv);

    // var walls = 0;
    // for (var xi = -5; xi < 5; xi++) {
    //     for (var yi = -5; yi < 5; yi++) {
    //         var this_uv = uv;
    //         if (is_wall(uv + (0.001 * vec2<f32>(f32(xi), f32(yi))))) {
    //             walls += 1;
    //         }
    //     }
    // }
    // let shadow = f32(walls) * 0.01;

    // return base_colour * (1.0 - shadow);

    let depth = bevy_pbr::prepass_utils::prepass_depth(in.uv, 0);
    return vec4(depth, depth, depth, 1.0);
}

fn is_wall(uv: vec2<f32>) -> bool {
    let base_colour = textureSample(screen_texture, texture_sampler, uv);
    if (base_colour.r + base_colour.g + base_colour.b) > 2.9 {
        return true;
    }
    return false;
}
#import bevy_core_pipeline::fullscreen_vertex_shader::FullscreenVertexOutput
#import bevy_render::view::View

struct AmbientLight {
    colour: vec3<f32>
}

struct PointLight {
    center: vec2<f32>,
    radius: f32,
    colour: vec4<f32>,
    intensity: f32,
    falloff: f32
}

struct CircularOccluder {
    center: vec2<f32>,
    radius: f32,
}


fn square(x: f32) -> f32 {
    return x * x;
}

// Calculate the signed distance from a point to a circle
fn signedDistanceCircle(point_pos: vec2f, radius: f32) -> f32 {
  return length(point_pos) - radius;
}

// Calculate the signed distance from a point to an occluder
fn signedDistanceOccluder(point_pos: vec2f, occluder: CircularOccluder) -> f32 {
    return signedDistanceCircle(point_pos - occluder.center, occluder.radius);
}

// Convert world coordinates to normalised device coordinates (NDC)
fn world_to_ndc(world_position: vec2<f32>, view_projection: mat4x4<f32>) -> vec2<f32> {
    return (view_projection * vec4<f32>(world_position, 0.0, 1.0)).xy;
}

// Convert NDC to screen coordinates
fn ndc_to_screen(ndc: vec2<f32>, screen_size: vec2<f32>) -> vec2<f32> {
    let screen_position: vec2<f32> = (ndc + 1.0) * 0.5 * screen_size;
    return vec2(screen_position.x, (screen_size.y - screen_position.y));
}

// Convert world coordinates directly to screen coordinates
fn world_to_screen(
    world_position: vec2<f32>,
    screen_size: vec2<f32>,
    view_projection: mat4x4<f32>
) -> vec2<f32> {
    return ndc_to_screen(world_to_ndc(world_position, view_projection), screen_size);
}

// Calculate the scale factor based on the view
fn scale_factor(view: View) -> f32 {
    let screen_size =
        2.0 * vec2f(view.inverse_projection[0][0], view.inverse_projection[1][1]);
    return screen_size.y / view.viewport.w;
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
    // Setup aggregate colour from light sources to multiply the main texture by.
    var light_colour = vec3(1.0);

    var total_brightness = 0.1;

    let the_occluder = CircularOccluder(world_to_screen(vec2(272.0, 272.0), view.viewport.zw, view.view_proj), 25.0);

    let ray_origin = vertex.position.xy;

    for (var i = 0u; i < arrayLength(&point_lights); i++) {
        let light = point_lights[i];
        let light_pos = world_to_screen(light.center, view.viewport.zw, view.view_proj);

        let ray_dir = normalize(light_pos - ray_origin);

        var ray_progress = 0.0;
        for (var j = 0u; j < 64u; j++) {
            let light_distance = distance(ray_origin, light_pos);
            if (ray_progress > light_distance) {

                let attenuation = attenuation(
                    light_distance,
                    light.radius,
                    light.intensity,
                    light.falloff
                );

                total_brightness += attenuation;


                break;
            }

            let scene_dist = signedDistanceOccluder(ray_origin + ray_dir * ray_progress, the_occluder);
            if (scene_dist <= 0.0) {
                break;
            }

            ray_progress += scene_dist;
        }
    }

    return textureSample(screen_texture, texture_sampler, vertex.uv)
        * vec4(light_colour, 1.0) * clamp(total_brightness, 0.0, 1.0);
}

// Compute light attenuation
fn attenuation(distance: f32, radius: f32, intensity: f32, falloff: f32) -> f32 {
    let s = distance / radius;
    if (s > 1.0) {
        return 0.0;
    }
    let s2 = square(s);
    return intensity * square(1 - s2) / (1 + falloff * s2);
}

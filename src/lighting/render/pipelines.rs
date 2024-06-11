#[allow(unused_imports)]
use bevy::{
    core_pipeline::fullscreen_vertex_shader::fullscreen_shader_vertex_state,
    prelude::*,
    render::{
        render_resource::{
            binding_types::{sampler, storage_buffer_read_only, texture_2d, uniform_buffer},
            BindGroupLayout, BindGroupLayoutEntries, CachedRenderPipelineId, ColorTargetState,
            ColorWrites, FragmentState, MultisampleState, PipelineCache, PrimitiveState,
            RenderPipelineDescriptor, Sampler, SamplerBindingType, SamplerDescriptor, ShaderStages,
            TextureFormat, TextureSampleType,
        },
        renderer::RenderDevice,
        texture::BevyDefault,
        view::ViewUniform,
    },
};

use super::{
    super::constants::{LIGHTING_BIND_GROUP_LAYOUT, LIGHTING_PIPELINE},
    buffers::{GpuCircularOccluder2D, GpuPointLight2D},
    extract::ExtractedAmbientLight2D,
};

#[derive(Resource)]
pub struct LightingPipeline {
    pub bind_group_layout: BindGroupLayout,
    pub sampler: Sampler,
    pub pipeline_id: CachedRenderPipelineId,
}

impl FromWorld for LightingPipeline {
    fn from_world(world: &mut World) -> Self {
        // Get the RenderDevice resource
        let render_device = world.resource::<RenderDevice>();

        // Create the BindGroupLayout
        let bind_group_layout = render_device.create_bind_group_layout(
            LIGHTING_BIND_GROUP_LAYOUT,
            &BindGroupLayoutEntries::sequential(
                ShaderStages::FRAGMENT,
                (
                    texture_2d(TextureSampleType::Float { filterable: true }),
                    sampler(SamplerBindingType::Filtering),
                    uniform_buffer::<ViewUniform>(true),
                    uniform_buffer::<ExtractedAmbientLight2D>(true),
                    storage_buffer_read_only::<Vec<GpuPointLight2D>>(false),
                    storage_buffer_read_only::<Vec<GpuCircularOccluder2D>>(false),
                ),
            ),
        );

        // Create the Sampler
        let sampler = render_device.create_sampler(&SamplerDescriptor::default());

        // Load the shader
        let shader = world
            .resource::<AssetServer>()
            .load("shaders/lighting.wgsl");

        // Queue the render pipeline
        let pipeline_id =
            world
                .resource_mut::<PipelineCache>()
                .queue_render_pipeline(RenderPipelineDescriptor {
                    label: Some(LIGHTING_PIPELINE.into()),
                    layout: vec![bind_group_layout.clone()],
                    vertex: fullscreen_shader_vertex_state(),
                    fragment: Some(FragmentState {
                        shader,
                        shader_defs: vec![],
                        entry_point: "fragment".into(),
                        targets: vec![Some(ColorTargetState {
                            format: TextureFormat::bevy_default(),
                            blend: None,
                            write_mask: ColorWrites::ALL,
                        })],
                    }),
                    primitive: PrimitiveState::default(),
                    depth_stencil: None,
                    multisample: MultisampleState::default(),
                    push_constant_ranges: vec![],
                });

        Self {
            bind_group_layout,
            sampler,
            pipeline_id,
        }
    }
}

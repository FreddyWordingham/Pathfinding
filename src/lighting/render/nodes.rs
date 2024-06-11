use bevy::{
    prelude::*,
    render::{
        extract_component::{ComponentUniforms, DynamicUniformIndex},
        render_graph::ViewNode,
        render_resource::{
            BindGroupEntries, Operations, PipelineCache, RenderPassColorAttachment,
            RenderPassDescriptor,
        },
        view::{ViewTarget, ViewUniformOffset, ViewUniforms},
    },
};

use super::{
    super::constants::{LIGHTING_BIND_GROUP, LIGHTING_PASS},
    buffers::{GpuCircularOccluders2DBuffer, GpuPointLights2DBuffer},
    extract::ExtractedAmbientLight2D,
    LightingPipeline,
};

#[derive(Default)]
pub struct LightingNode;

impl ViewNode for LightingNode {
    type ViewQuery = (
        &'static ViewTarget,
        &'static DynamicUniformIndex<ExtractedAmbientLight2D>,
        &'static ViewUniformOffset,
    );

    fn run<'w>(
        &self,
        _graph: &mut bevy::render::render_graph::RenderGraphContext,
        render_context: &mut bevy::render::renderer::RenderContext<'w>,
        (view_target, ambient_light, view_offset): bevy::ecs::query::QueryItem<'w, Self::ViewQuery>,
        world: &'w World,
    ) -> Result<(), bevy::render::render_graph::NodeRunError> {
        // Get the LightingPipeline resource
        let lighting_pipeline = world.resource::<LightingPipeline>();

        // Get the PipelineCache resource
        let pipeline_cache = world.resource::<PipelineCache>();

        // Fetch the render pipeline from the pipeline cache
        let Some(pipeline) = pipeline_cache.get_render_pipeline(lighting_pipeline.pipeline_id)
        else {
            return Ok(());
        };

        // Fetch the view uniform binding
        let Some(view_uniform_binding) = world.resource::<ViewUniforms>().uniforms.binding() else {
            return Ok(());
        };
        // Fetch the ambient light uniform binding
        let Some(ambient_light_uniform) = world
            .resource::<ComponentUniforms<ExtractedAmbientLight2D>>()
            .uniforms()
            .binding()
        else {
            return Ok(());
        };
        // Fetch the GPU point light buffer binding
        let Some(point_light_buffer) = world.resource::<GpuPointLights2DBuffer>().buffer.binding()
        else {
            return Ok(());
        };
        // Fetch the GPU circular occluder buffer binding
        let Some(circular_occluder_buffer) = world
            .resource::<GpuCircularOccluders2DBuffer>()
            .buffer
            .binding()
        else {
            return Ok(());
        };

        // Get the post-process target for the view
        let post_process = view_target.post_process_write();

        // Create the bind group with necessary bindings
        let bind_group = render_context.render_device().create_bind_group(
            LIGHTING_BIND_GROUP,
            &lighting_pipeline.bind_group_layout,
            &BindGroupEntries::sequential((
                post_process.source,
                &lighting_pipeline.sampler,
                view_uniform_binding,
                ambient_light_uniform,
                point_light_buffer,
                circular_occluder_buffer,
            )),
        );

        // Begin the render pass
        let mut render_pass = render_context.begin_tracked_render_pass(RenderPassDescriptor {
            label: Some(LIGHTING_PASS),
            color_attachments: &[Some(RenderPassColorAttachment {
                view: post_process.destination,
                resolve_target: None,
                ops: Operations::default(),
            })],
            depth_stencil_attachment: None,
            timestamp_writes: None,
            occlusion_query_set: None,
        });

        // Set the render pipeline and bind group
        render_pass.set_render_pipeline(pipeline);
        render_pass.set_bind_group(0, &bind_group, &[view_offset.offset, ambient_light.index()]);
        render_pass.draw(0..3, 0..1);

        Ok(())
    }
}

use bevy::{
    prelude::*,
    render::{
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
    LightingPipeline,
};

#[derive(Default)]
pub struct LightingNode;

impl ViewNode for LightingNode {
    type ViewQuery = (&'static ViewTarget, &'static ViewUniformOffset);

    fn run<'w>(
        &self,
        _graph: &mut bevy::render::render_graph::RenderGraphContext,
        render_context: &mut bevy::render::renderer::RenderContext<'w>,
        (view_target, view_offset): bevy::ecs::query::QueryItem<'w, Self::ViewQuery>,
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
        render_pass.set_bind_group(0, &bind_group, &[view_offset.offset]); // Add more binding indices here
        render_pass.draw(0..3, 0..1);

        Ok(())
    }
}

use bevy::{
    core_pipeline::core_2d::graph::{Core2d, Node2d::MainPass},
    prelude::*,
    render::{
        extract_component::UniformComponentPlugin,
        render_graph::{RenderGraphApp, ViewNodeRunner},
        Render, RenderApp, RenderSet,
    },
};

use super::{components::*, render::*, systems::*};

pub struct LightingPlugin;

impl Plugin for LightingPlugin {
    fn build(&self, app: &mut App) {
        app.add_plugins(UniformComponentPlugin::<ExtractedAmbientLight2D>::default())
            .register_type::<AmbientLight2D>()
            .register_type::<PointLight2D>()
            .register_type::<CircularOccluder2D>();

        let Ok(render_app) = app.get_sub_app_mut(RenderApp) else {
            return;
        };

        render_app
            .add_systems(
                ExtractSchedule,
                (
                    extract_ambient_lights,
                    extract_point_lights,
                    extract_circular_occluders,
                ),
            )
            .add_systems(
                Render,
                (prepare_point_lights, prepare_circle_occluders).in_set(RenderSet::Prepare),
            )
            .add_render_graph_node::<ViewNodeRunner<LightingNode>>(Core2d, LightingPass)
            .add_render_graph_edge(Core2d, MainPass, LightingPass);
    }

    fn finish(&self, app: &mut App) {
        let Ok(render_app) = app.get_sub_app_mut(RenderApp) else {
            return;
        };

        render_app
            .init_resource::<LightingPipeline>()
            .init_resource::<GpuPointLights2DBuffer>()
            .init_resource::<GpuCircularOccluders2DBuffer>();
    }
}

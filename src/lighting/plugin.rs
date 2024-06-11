use bevy::{prelude::*, render::RenderApp};

use super::render::LightingPipeline;

pub struct LightingPlugin;

impl Plugin for LightingPlugin {
    fn build(&self, _app: &mut App) {}

    fn finish(&self, app: &mut App) {
        let Ok(render_app) = app.get_sub_app_mut(RenderApp) else {
            return;
        };

        render_app.init_resource::<LightingPipeline>();
    }
}

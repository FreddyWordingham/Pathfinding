use bevy::{
    ecs::component::Component, math::Vec4, render::render_resource::ShaderType,
    transform::components::GlobalTransform,
};

#[derive(Default, Clone, Component, ShaderType)]
pub struct ExtractedAmbientLight2D {
    pub colour: Vec4,
}

#[derive(Component)]
pub struct ExtractedPointLight2D {
    pub transform: GlobalTransform,
    pub radius: f32,
    pub colour: Vec4,
    pub intensity: f32,
    pub falloff: f32,
}

#[derive(Component)]
pub struct ExtractedCircularOccluder2D {
    pub transform: GlobalTransform,
    pub radius: f32,
}

use bevy::{
    prelude::*,
    render::render_resource::{ShaderType, StorageBuffer},
};

#[derive(Default, Clone, ShaderType)]
pub struct GpuPointLight2D {
    pub center: Vec2,
    pub radius: f32,
    pub colour: Vec4,
    pub intensity: f32,
    pub falloff: f32,
}

#[derive(Default, Resource)]
pub struct GpuPointLights2DBuffer {
    pub buffer: StorageBuffer<Vec<GpuPointLight2D>>,
}

#[derive(Default, Clone, ShaderType)]
pub struct GpuCircularOccluder2D {
    pub center: Vec2,
    pub radius: f32,
}

#[derive(Default, Resource)]
pub struct GpuCircularOccluders2DBuffer {
    pub buffer: StorageBuffer<Vec<GpuCircularOccluder2D>>,
}

use bevy::{
    prelude::*,
    render::render_resource::{ShaderType, StorageBuffer},
};

#[derive(Default, Clone, ShaderType)]
pub struct GpuPointLight {
    pub center: Vec2,
    pub radius: f32,
    pub color: Vec4,
    pub intensity: f32,
    pub falloff: f32,
}

#[derive(Default, Resource)]
pub struct GpuPointLightsBuffer {
    pub buffer: StorageBuffer<Vec<GpuPointLight>>,
}

#[derive(Default, Clone, ShaderType)]
pub struct GpuCircularOccluder {
    pub center: Vec2,
    pub radius: f32,
}

#[derive(Default, Resource)]
pub struct GpuCircularOccludersBuffer {
    pub buffer: StorageBuffer<Vec<GpuCircularOccluder>>,
}

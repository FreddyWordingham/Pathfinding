use bevy::prelude::*;
use bevy::render::{extract_component::ExtractComponent, render_resource::ShaderType};

// This is the component that will get passed to the shader
#[derive(Component, Default, Clone, Copy, ExtractComponent, ShaderType)]
pub struct PostProcessSettings {
    pub intensity: f32,
}

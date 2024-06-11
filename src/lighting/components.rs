use bevy::prelude::*;

#[derive(Component, Reflect)]
pub struct AmbientLight {
    pub colour: Color,
    pub brightness: f32,
}

#[derive(Component, Reflect)]
pub struct PointLight {
    pub colour: Color,
    pub brightness: f32,
    pub radius: f32,
    pub falloff: f32,
}

#[derive(Component, Reflect)]
pub struct CircularOccluder {
    pub radius: f32,
}

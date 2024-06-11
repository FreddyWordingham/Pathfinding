use bevy::prelude::*;

#[derive(Component, Reflect)]
pub struct AmbientLight2D {
    pub colour: Color,
    pub brightness: f32,
}

#[derive(Component, Reflect)]
pub struct PointLight2D {
    pub colour: Color,
    pub brightness: f32,
    pub radius: f32,
    pub falloff: f32,
}

#[derive(Component, Reflect)]
pub struct CircularOccluder2D {
    pub radius: f32,
}

use bevy_render::prelude::*;
use bevy_rapier3d::prelude::*;
use bevy_render::prelude::shape::UVSphere;

pub fn ball(ball: &Ball, config: &RapierConfiguration) -> Mesh {
    Mesh::from(UVSphere {
        radius: ball.radius * config.scale,
        ..Default::default()
    })
}
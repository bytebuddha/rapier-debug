use bevy_ecs::prelude::*;
use bevy_render::prelude::*;

#[derive(Component)]
pub struct ColliderWireframe;

#[derive(Component)]
pub struct ColliderWireframe2d;

#[derive(Component)]
pub struct ColliderDebugRender {
    pub color: Color,
    pub fill_alpha: f32
}

impl ColliderDebugRender {
    pub fn new(color: Color) -> ColliderDebugRender {
        ColliderDebugRender {
            color,
            ..Default::default()
        }
    }
}

impl Default for ColliderDebugRender {
    fn default() -> ColliderDebugRender {
        ColliderDebugRender {
            color: Color::RED,
            fill_alpha: 0.25
        }
    }
}

#[derive(Component)]
pub struct ColliderDebugEntity(pub Option<Entity>);
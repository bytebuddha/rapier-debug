use bevy::prelude::*;
use bevy_rapier2d::prelude::*;
use rapier_debug::ColliderDebugRender;

/// set up a simple 3D scene
pub fn setup_environment(
    mut commands: Commands,
) {
    commands
        .spawn_bundle(ColliderBundle {
            shape: ColliderShape::cuboid(100.0, 10.0).into(),
            position: Vec2::new(0.0, -100.0).into(),
            ..Default::default()
        })
        .insert(ColliderDebugRender::new(Color::TEAL))
        .insert(ColliderPositionSync::Discrete);

    // camera
    commands.spawn_bundle(OrthographicCameraBundle::new_2d());
}

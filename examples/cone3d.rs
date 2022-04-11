use bevy::prelude::*;
use bevy_rapier3d::prelude::*;
use bevy_inspector_egui::WorldInspectorPlugin;
use rapier_3debug::{ColliderDebugRender, RapierDebugPlugin};

mod common;

fn main() {
    App::new()
        .insert_resource(Msaa { samples: 4 })
        .add_plugins(DefaultPlugins)
        .add_plugin(RapierDebugPlugin)
        .add_plugin(RapierPhysicsPlugin::<()>::default())
        .add_plugin(WorldInspectorPlugin::new())
        .add_startup_system(common::setup_environment)
        .add_startup_system(setup)
        .run();
}

/// set up a simple 3D scene
fn setup(
    mut commands: Commands,
) {
    commands.spawn_bundle(RigidBodyBundle {
        body_type: RigidBodyType::Dynamic.into(),
        position: [
            -Vec3::from(common::BOWL_SIZE).x / 4.0,
            5.0,
            Vec3::from(common::BOWL_SIZE).z / 2.0,
        ].into(),
        ..Default::default()
    }).insert_bundle(ColliderBundle {
        shape: ColliderShape::cone(
            1.0,
            0.5
        ).into(),
        ..Default::default()
    }).insert(ColliderDebugRender::new(Color::PURPLE))
        .insert(RigidBodyPositionSync::Discrete);
}
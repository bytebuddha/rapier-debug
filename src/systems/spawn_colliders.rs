use bevy::prelude::*;
#[cfg(feature = "dim3")]
use bevy_rapier3d::prelude::*;
#[cfg(feature = "dim2")]
use bevy_rapier2d::prelude::*;

use crate::{ColliderDebugRender, ColliderDebugEntity};

pub fn spawn_colliders(
    mut commands: Commands,
    mut meshes: ResMut<Assets<Mesh>>,
    config: Res<RapierConfiguration>,
    #[cfg(feature = "dim3")]
    mut materials: ResMut<Assets<bevy::pbr::StandardMaterial>>,
    #[cfg(feature = "dim2")]
    mut materials: ResMut<Assets<bevy::sprite::ColorMaterial>>,
    query: Query<
        (
            Entity, &ColliderShapeComponent, &ColliderTypeComponent, &ColliderDebugRender, &ColliderPositionComponent,
            Option<&RigidBodyPositionSync>, Option<&ColliderPositionSync>,
            Option<&RigidBodyPositionComponent>
        ),
        Without<ColliderDebugEntity>
    >
) {
    for (entity, shape, ty, debug, co_pos, rb_sync, co_sync, rb_pos) in query.iter() {
        let transform = {
            if co_sync.is_some() {
                Transform::identity()
            } else if rb_sync.is_some() {
                if let Some(rb_pos) = rb_pos {
                    rigid_body_transform(rb_pos, co_pos)
                } else {
                    Default::default()
                }
            } else {
                collider_transform(co_pos)
            }
        };
        let mut cube_color = debug.color;
        cube_color.set_a(debug.fill_alpha);
        if let Some(mesh) = collider_mesh(shape, &config) {
            #[cfg(feature = "dim3")]
            let debug_entity = commands.spawn_bundle((
                transform,
                GlobalTransform::identity(),
                bevy::core::Name::new("Debug Collider")
                )).with_children(|cmds| {
                    let mut wire_mesh = mesh.clone();
                    crate::mesh_attributes::WireframeMeshGenerator::compute_barycentric(&mut wire_mesh);
                    crate::mesh_attributes::WireframeMeshGenerator::color_vertex(&mut wire_mesh, debug.color);
                    let dashes = if ty.0 == ColliderType::Sensor {
                        [4.0, 0.4]
                    } else {
                        [1.0, 1.0]
                    };
                    crate::mesh_attributes::WireframeMeshGenerator::add_dash_to_vertex(&mut wire_mesh, dashes);
                    cmds.spawn_bundle(bevy::prelude::PbrBundle {
                        mesh: meshes.add(mesh),
                        material: materials.add(bevy::prelude::StandardMaterial {
                            unlit: true,
                            base_color: cube_color,
                            alpha_mode: bevy::pbr::AlphaMode::Blend,
                            ..Default::default()
                        }),
                        transform,
                        ..Default::default()
                    }).insert_bundle((
                        bevy::pbr::NotShadowCaster,
                        bevy::core::Name::new("Fill"),
                    ));
                    cmds.spawn_bundle((
                        Transform::identity(),
                        GlobalTransform::identity(),
                        Visibility::default(),
                        meshes.add(wire_mesh),
                        ComputedVisibility::default(),
                        bevy::pbr::NotShadowCaster,
                        bevy::core::Name::new("Wireframe"),
                        crate::ColliderWireframe
                    ));
                }).id();
            #[cfg(feature = "dim2")]
            let debug_entity = commands.spawn_bundle((
                    transform,
                    GlobalTransform::identity(),
                    Name::new("Debug Collider")
                )).with_children(|cmds| {
                    let mut wire_mesh = mesh.clone();
                    crate::mesh_attributes::WireframeMeshGenerator::compute_barycentric(&mut wire_mesh);
                    crate::mesh_attributes::WireframeMeshGenerator::color_vertex(&mut wire_mesh, debug.color);
                    let dashes = if ty.0 == ColliderType::Sensor {
                        [4.0, 0.4]
                    } else {
                        [1.0, 1.0]
                    };
                    crate::mesh_attributes::WireframeMeshGenerator::add_dash_to_vertex(&mut wire_mesh, dashes);
                    cmds.spawn_bundle((
                        bevy::sprite::Mesh2dHandle(meshes.add(wire_mesh)),
                        Transform::identity(),
                        GlobalTransform::identity(),
                        crate::ColliderWireframe2d,
                        Visibility::default(),
                        ComputedVisibility::default(),
                        Name::new("Wireframe")
                    ));
                    cmds.spawn_bundle(ColorMesh2dBundle {
                        mesh: bevy::sprite::Mesh2dHandle(meshes.add(mesh)),
                        material: materials.add(cube_color.into()),
                        ..Default::default()
                    }).insert(Name::new("Fill"));
                }).id();

            commands.entity(entity)
                .insert(ColliderDebugEntity(Some(debug_entity)))
                .push_children(&[debug_entity]);
        } else {
            commands.entity(entity).insert(ColliderDebugEntity(None));
        }
    }
}

fn collider_mesh(shape: &ColliderShapeComponent, config: &RapierConfiguration) -> Option<Mesh> {
    match shape.0.shape_type() {
        #[cfg(feature = "dim3")]
        ShapeType::Ball => {
            let ball = shape.0.as_ball().unwrap();
            Some(crate::mesh::ball(ball, config))
        },
        #[cfg(feature = "dim2")]
        ShapeType::Ball => {
            let ball = shape.0.as_ball().unwrap();
            Some(crate::mesh2d::ball(ball, config))
        },
        #[cfg(feature = "dim3")]
        ShapeType::Cuboid => {
            let cuboid = shape.0.as_cuboid().unwrap();
            Some(crate::mesh::cuboid(cuboid, config))
        },
        #[cfg(feature = "dim2")]
        ShapeType::Cuboid => {
            let cuboid = shape.0.as_cuboid().unwrap();
            Some(crate::mesh2d::cuboid(cuboid, config))
        },
        #[cfg(feature = "dim3")]
        ShapeType::Capsule => {
            let capsule = shape.0.as_capsule().unwrap();
            Some(crate::mesh::capsule(capsule, config))
        },
        #[cfg(feature = "dim2")]
        ShapeType::Capsule => {
            let capsule = shape.0.as_capsule().unwrap();
            Some(crate::mesh2d::capsule(capsule, config))
        },
        #[cfg(feature = "dim3")]
        ShapeType::TriMesh => {
            let trimesh = shape.0.as_trimesh().unwrap();
            Some(crate::mesh::trimesh(trimesh))
        },
        #[cfg(feature = "dim2")]
        ShapeType::TriMesh => {
            let trimesh = shape.0.as_trimesh().unwrap();
            Some(crate::mesh2d::trimesh(trimesh))
        },
        #[cfg(feature = "dim3")]
        ShapeType::Cylinder => {
            let cylinder = shape.0.as_cylinder().unwrap();
            Some(crate::mesh::cylinder(cylinder))
        },
        #[cfg(feature = "dim3")]
        ShapeType::Cone => {
            let cone = shape.0.as_cone().unwrap();
            Some(crate::mesh::cone(cone))
        },
        _ => None
    }
}

#[cfg(feature = "dim3")]
fn collider_transform(co_pos: &ColliderPosition) -> Transform {
    let mut transform = Transform::from_translation(co_pos.translation.into());
    transform.rotation = co_pos.rotation.into();
    transform
}

#[cfg(feature = "dim3")]
fn rigid_body_transform(rb_pos: &RigidBodyPosition, co_pos: &ColliderPosition) -> Transform {
    let mut co_transform = Transform::from_translation(Vec3::from(co_pos.translation) - Vec3::from(rb_pos.position.translation));
    co_transform.rotation = Quat::from(co_pos.rotation);
    co_transform
}

#[cfg(feature = "dim2")]
fn collider_transform(co_pos: &ColliderPosition) -> Transform {
    Transform::from_xyz(
        co_pos.translation.x,
        co_pos.translation.y,
        1.0
    )
}

#[cfg(feature = "dim2")]
fn rigid_body_transform(rb_pos: &RigidBodyPosition, co_pos: &ColliderPosition) -> Transform {
    let pos = Vec2::from(co_pos.translation) - Vec2::from(rb_pos.position.translation);
    Transform::from_xyz(pos.x, pos.y, 1.0)
}
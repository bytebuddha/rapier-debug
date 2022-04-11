use bevy::prelude::*;
use bevy_rapier3d::prelude::*;
use rapier_debug::ColliderDebugRender;

/// set up a simple 3D scene
pub fn setup_environment(
    mut commands: Commands,
) {
    use std::f32::consts::TAU;
    use bevy_rapier3d::na::Point3;
    let mut vertices: Vec<Point3<f32>> = Vec::new();
    let mut indices: Vec<[u32; 3]> = Vec::new();
    let segments = 32;
    let bowl_size = Vec3::new(10.0, 3.0, 10.0);
    for ix in 0..=segments {
        for iz in 0..=segments {
            // Map x and y into range [-1.0, 1.0];
            let shifted_z = (iz as f32 / segments as f32 - 0.5) * 2.0;
            let shifted_x = (ix as f32 / segments as f32 - 0.5) * 2.0;
            // Clamp radius at 1.0 or lower so the bowl has a flat lip near the corners.
            let clamped_radius = (shifted_z.powi(2) + shifted_x.powi(2)).sqrt().min(1.0);
            let x = shifted_x * bowl_size.x / 2.0;
            let z = shifted_z * bowl_size.z / 2.0;
            let y = ((clamped_radius - 0.5) * TAU / 2.0).sin() * bowl_size.y / 2.0;
            vertices.push(Point3::new(x, y, z));
        }
    }
    for ix in 0..segments {
        for iz in 0..segments {
            // Start of the two relevant rows of vertices.
            let row0 = ix * (segments + 1);
            let row1 = (ix + 1) * (segments + 1);
            // Two triangles making up a not-very-flat quad for each segment of the bowl.
            indices.push([row0 + iz + 0, row0 + iz + 1, row1 + iz + 0]);
            indices.push([row1 + iz + 0, row0 + iz + 1, row1 + iz + 1]);
        }
    }
    // Position so ramp connects smoothly
    // to one edge of the lip of the bowl.
    let collider = ColliderBundle {
        shape: ColliderShape::trimesh(vertices, indices).into(),
        position: [
            -bowl_size.x / 4.0,
            -bowl_size.y / 4.0,
            bowl_size.z / 2.0,
        ].into(),
        ..Default::default()
    };
    commands
        .spawn_bundle(collider)
        .insert(ColliderDebugRender::new(Color::TEAL))
        .insert(ColliderPositionSync::Discrete);

    // camera
    commands.spawn_bundle(PerspectiveCameraBundle {
        transform: Transform::from_xyz(-14.0, 7.0, 4.0).looking_at(Vec3::new(5.0, 0.0, 5.0), Vec3::Y),
        ..Default::default()
    });

    // light
    commands.spawn_bundle(PointLightBundle {
        point_light: PointLight {
            intensity: 1500.0,
            shadows_enabled: true,
            ..Default::default()
        },
        transform: Transform::from_xyz(4.0, 8.0, 4.0),
        ..Default::default()
    });
}

use bevy::prelude::*;
use bevy_rapier3d::prelude::*;
use bevy_render::mesh::Indices;
use bevy_render::render_resource::PrimitiveTopology;

pub fn cylinder(c: &Cylinder) -> Mesh {
    assert!(c.radius > 0.0 && c.half_height * 2.0 > 0.0);

    let count = 142;
    let mut positions = Vec::with_capacity(count);
    let step = std::f32::consts::PI * 2.0 / 20.0;
    let mut add_ring = |height, with_center| {
        if with_center {
            positions.push([0.0, height, 0.0]);
        }
        for j in 0..20 {
            let theta = step * j as f32;
            positions.push([theta.cos() * c.radius, height, theta.sin() * c.radius]);
        }
    };

    // Shaft vertices
    let h_step = c.half_height/ 2.0;
    for i in 0..5 {
        add_ring(c.half_height - h_step * i as f32, false);
    }

    // Top vertices
    let top_offset = 100;
    add_ring(c.half_height * 2.0 * 0.5, true);

    // Bottom vertices
    let bottom_offset = top_offset + 21;
    add_ring(-c.half_height, true);
    assert_eq!(positions.len(), count);

    // Index buffer
    let index_count = (6 * 4 * 20) + 6 * 20;
    let mut indices = Vec::with_capacity(index_count);
    // Shaft quads
    for i in 0..4 {
        let base1 = 20 * i;
        let base2 = base1 + 20;
        for j in 0..20 {
            let j1 = (j + 1) % 20;
            indices.extend([base2 + j, base1 + j1, base1 + j].iter().copied());
            indices.extend([base2 + j, base2 + j1, base1 + j1].iter().copied());
        }
    }

    // Top circle triangles
    for j in 0..20 {
        let j1 = (j + 1) % 20;
        let base = top_offset + 1;
        indices.extend([top_offset, base + j, base + j1].iter().copied());
    }
    // Bottom circle triangles
    for j in 0..20 {
        let j1 = (j + 1) % 20;
        let base = bottom_offset + 1;
        indices.extend([bottom_offset, base + j1, base + j].iter().copied());
    }
    assert_eq!(indices.len(), index_count);

    // Shaft normals are their positions X&Z
    let mut normals = positions
        .iter()
        .map(|&p| {
            (Vec3::from(p) * Vec3::new(1.0, 0.0, 1.0))
                .normalize()
                .into()
        })
        .collect::<Vec<[f32; 3]>>();

    // Give the top and bottom of the cylinder a clear up/down normal
    for i in top_offset..bottom_offset {
        normals[i as usize] = [0.0, 1.0, 0.0];
    }
    for i in bottom_offset..count as u32 {
        normals[i as usize] = [0.0, -1.0, 0.0];
    }

    let uvs: Vec<[f32 ; 2]> = positions
        .iter()
        .map(|&p| [p[0] / c.radius, (p[1] + c.half_height) / (c.half_height * 2.0)])
        .collect();

    let mut mesh = Mesh::new(PrimitiveTopology::TriangleList);
    mesh.set_attribute(Mesh::ATTRIBUTE_POSITION, positions);
    mesh.set_attribute(Mesh::ATTRIBUTE_NORMAL, normals);
    mesh.set_attribute(Mesh::ATTRIBUTE_UV_0, uvs);
    mesh.set_indices(Some(Indices::U32(indices)));

    mesh
}
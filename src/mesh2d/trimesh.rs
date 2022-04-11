use bevy_math::prelude::*;
use bevy_render::prelude::*;
use bevy_rapier2d::prelude::*;
use bevy_render::mesh::{Indices, VertexAttributeValues};
use bevy_render::render_resource::PrimitiveTopology;

pub fn trimesh(trimesh: &TriMesh) -> Mesh {
    let mut mesh = Mesh::new(PrimitiveTopology::TriangleList);
    mesh.set_attribute(
        Mesh::ATTRIBUTE_POSITION,
        VertexAttributeValues::from(
            trimesh
                .vertices()
                .iter()
                .map(|vertex| [vertex.x, vertex.y])
                .collect::<Vec<_>>(),
        ),
    );
    // Compute vertex normals by averaging the normals
    // of every triangle they appear in.
    // NOTE: This is a bit shonky, but good enough for visualisation.
    let verts = trimesh.vertices();
    let mut normals: Vec<Vec2> = vec![Vec2::ZERO; trimesh.vertices().len()];
    for triangle in trimesh.indices().iter() {
        let ab = verts[triangle[1] as usize] - verts[triangle[0] as usize];
        let ac = verts[triangle[2] as usize] - verts[triangle[0] as usize];
        let normal = ab.cross(&ac);
        // Contribute this normal to each vertex in the triangle.
        for i in 0..3 {
            normals[triangle[i] as usize] += Vec2::new(normal.x, normal.y);
        }
    }
    let normals: Vec<[f32; 2]> = normals
        .iter()
        .map(|normal| {
            let normal = normal.normalize();
            [normal.x, normal.y]
        })
        .collect();
    mesh.set_attribute(Mesh::ATTRIBUTE_NORMAL, VertexAttributeValues::from(normals));
    // There's nothing particularly meaningful we can do
    // for this one without knowing anything about the overall topology.
    mesh.set_attribute(
        Mesh::ATTRIBUTE_UV_0,
        VertexAttributeValues::from(
            trimesh
                .vertices()
                .iter()
                .map(|_vertex| [0.0, 0.0])
                .collect::<Vec<_>>(),
        ),
    );
    mesh.set_indices(Some(Indices::U32(
        trimesh
            .indices()
            .iter()
            .flat_map(|triangle| triangle.iter())
            .cloned()
            .collect(),
    )));
    mesh
}
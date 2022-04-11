use bevy_render::prelude::*;
use bevy_rapier2d::prelude::*;
use bevy_render::mesh::Indices;
use bevy_render::render_resource::PrimitiveTopology;

pub fn ball(ball: &Ball, _config: &RapierConfiguration) -> Mesh {

    let mut positions = Vec::with_capacity(64);
    let mut normals = Vec::with_capacity(64);
    let mut uvs = Vec::with_capacity(64);

    let step = std::f32::consts::TAU / 64.0;
    for i in 0..64 {
        let theta = std::f32::consts::FRAC_PI_2 - i as f32 * step;
        let (sin, cos) = theta.sin_cos();

        positions.push([cos * ball.radius, sin * ball.radius, 0.0]);
        normals.push([0.0, 0.0, 1.0]);
        uvs.push([0.5 * (cos + 1.0), 1.0 - 0.5 * (sin + 1.0)]);
    }

    let mut indices = Vec::with_capacity(186);
    for i in 1..63 {
        indices.extend_from_slice(&[0, i + 1, i]);
    }

    let mut mesh = Mesh::new(PrimitiveTopology::TriangleList);
    mesh.set_attribute(Mesh::ATTRIBUTE_POSITION, positions);
    mesh.set_attribute(Mesh::ATTRIBUTE_NORMAL, normals);
    mesh.set_attribute(Mesh::ATTRIBUTE_UV_0, uvs);
    mesh.set_indices(Some(Indices::U32(indices)));
    mesh
}
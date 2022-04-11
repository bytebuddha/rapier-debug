use bevy::prelude::*;
use bevy_rapier2d::prelude::*;

pub fn cuboid(cuboid: &Cuboid, _config: &RapierConfiguration) -> Mesh {
    Mesh::from(bevy::render::prelude::shape::Quad::new(Vec2::new(
        cuboid.half_extents.x * 2.0,
        cuboid.half_extents.y * 2.0
    )))
}
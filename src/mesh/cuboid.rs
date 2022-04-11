use bevy_render::prelude::*;
use bevy_rapier3d::prelude::*;

pub fn cuboid(cuboid: &Cuboid, config: &RapierConfiguration) -> Mesh {
    Mesh::from(bevy_render::prelude::shape::Box::new(
        cuboid.half_extents.x * 2.0 * config.scale,
        cuboid.half_extents.y * 2.0 * config.scale,
        cuboid.half_extents.z * 2.0 * config.scale
    ))
}
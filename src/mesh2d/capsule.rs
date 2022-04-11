use bevy_render::prelude::*;
use bevy_rapier2d::prelude::*;
use bevy_render::mesh::shape::CapsuleUvProfile;

pub fn capsule(capsule: &Capsule, config: &RapierConfiguration) -> Mesh {
    Mesh::from(bevy::prelude::shape::Capsule {
        radius: capsule.radius * config.scale,
        rings: 0,
        depth: capsule.half_height() * config.scale / 4.0,
        latitudes: 8,
        longitudes: 18,
        uv_profile: CapsuleUvProfile::Aspect
    })
}
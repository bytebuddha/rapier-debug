mod plugin;
mod systems;
mod collider;
mod mesh_attributes;

#[cfg(feature = "dim3")]
mod wireframe_pipeline;
#[cfg(feature = "dim2")]
mod wireframe2d_pipeline;

#[cfg(feature = "dim3")]
pub mod mesh;
#[cfg(feature = "dim2")]
pub mod mesh2d;

#[cfg(feature = "dim3")]
pub use self::wireframe_pipeline::ColliderWireframePipeline;
#[cfg(feature = "dim2")]
pub use self::wireframe2d_pipeline::ColliderWireframe2dPipeline;

pub use self::collider::*;
pub use self::plugin::RapierDebugPlugin;

use bevy::prelude::*;
use bevy::reflect::TypeUuid;

#[cfg(feature = "dim3")]
pub const WIREFRAME_SHADER_HANDLE: HandleUntyped =
    HandleUntyped::weak_from_u64(Shader::TYPE_UUID, 3590091414685923376);

#[cfg(feature = "dim2")]
pub const WIREFRAME2D_SHADER_HANDLE: HandleUntyped =
    HandleUntyped::weak_from_u64(Shader::TYPE_UUID, 1597091414686923176);

#[cfg(feature = "dim3")]
pub type DrawColliderWireframes = (
    // Set the pipeline
    bevy::render::render_phase::SetItemPipeline,
    // Set the view uniform as bind group 0
    bevy::pbr::SetMeshViewBindGroup<0>,
    // Set the mesh uniform as bind group 1
    bevy::pbr::SetMeshBindGroup<1>,
    // Draw the mesh
    bevy::pbr::DrawMesh,
);

#[cfg(feature = "dim2")]
pub type DrawColliderWireframes2d = (
    // Set the pipeline
    bevy::render::render_phase::SetItemPipeline,
    // Set the view uniform as bind group 0
    bevy::sprite::SetMesh2dViewBindGroup<0>,
    // Set the mesh uniform as bind group 1
    bevy::sprite::SetMesh2dBindGroup<1>,
    // Draw the mesh
    bevy::sprite::DrawMesh2d,
);
use bevy::prelude::*;
use bevy_render::render_asset::RenderAssets;
use bevy_render::render_phase::{DrawFunctions, RenderPhase};
use bevy_render::render_resource::{RenderPipelineCache, SpecializedPipelines};

#[cfg(feature = "dim3")]
pub fn queue_wireframes(
    opaque_3d_draw_functions: Res<DrawFunctions<bevy::core_pipeline::Transparent3d>>,
    render_meshes: Res<RenderAssets<Mesh>>,
    wireframe_pipeline: Res<crate::ColliderWireframePipeline>,
    mut pipeline_cache: ResMut<RenderPipelineCache>,
    mut specialized_pipelines: ResMut<SpecializedPipelines<crate::ColliderWireframePipeline>>,
    msaa: Res<Msaa>,
    material_meshes: Query<(Entity, &Handle<Mesh>, &bevy::pbr::MeshUniform), With<crate::ColliderWireframe>>,
    mut views: Query<(&bevy::render::view::ExtractedView, &mut RenderPhase<bevy::core_pipeline::Transparent3d>)>,
) {
    let draw_custom = opaque_3d_draw_functions
        .read()
        .get_id::<crate::DrawColliderWireframes>()
        .unwrap();

    let key =
        bevy::pbr::MeshPipelineKey::from_msaa_samples(msaa.samples) | bevy::pbr::MeshPipelineKey::TRANSPARENT_MAIN_PASS;
    for (view, mut transparent_phase) in views.iter_mut() {
        let view_matrix = view.transform.compute_matrix();
        let view_row_2 = view_matrix.row(2);

        let add_render_phase =
            |(entity, mesh_handle, mesh_uniform): (Entity, &Handle<Mesh>, &bevy::pbr::MeshUniform)| {
                if let Some(mesh) = render_meshes.get(mesh_handle) {
                    let key =
                        key | bevy::pbr::MeshPipelineKey::from_primitive_topology(mesh.primitive_topology);
                    transparent_phase.add(bevy::core_pipeline::Transparent3d {
                        entity,
                        pipeline: specialized_pipelines.specialize(
                            &mut pipeline_cache,
                            &wireframe_pipeline,
                            key,
                        ),
                        draw_function: draw_custom,
                        distance: view_row_2.dot(mesh_uniform.transform.col(3)),
                    });
                }
            };

        material_meshes.iter().for_each(add_render_phase);
    }
}

#[allow(clippy::too_many_arguments)]
#[cfg(feature = "dim2")]
pub fn queue_wireframes2d(
    transparent_draw_functions: Res<DrawFunctions<bevy::core_pipeline::Transparent2d>>,
    colored_mesh2d_pipeline: Res<crate::ColliderWireframe2dPipeline>,
    mut pipelines: ResMut<SpecializedPipelines<crate::ColliderWireframe2dPipeline>>,
    mut pipeline_cache: ResMut<RenderPipelineCache>,
    msaa: Res<Msaa>,
    render_meshes: Res<RenderAssets<Mesh>>,
    colored_mesh2d: Query<(&bevy::sprite::Mesh2dHandle, &bevy::sprite::Mesh2dUniform), With<crate::ColliderWireframe2d>>,
    mut views: Query<(&bevy::render::view::VisibleEntities, &mut RenderPhase<bevy::core_pipeline::Transparent2d>)>,
) {
    if colored_mesh2d.is_empty() {
        return;
    }
    // Iterate each view (a camera is a view)
    for (visible_entities, mut transparent_phase) in views.iter_mut() {
        let draw_colored_mesh2d = transparent_draw_functions
            .read()
            .get_id::<crate::DrawColliderWireframes2d>()
            .unwrap();

        let mesh_key = bevy::sprite::Mesh2dPipelineKey::from_msaa_samples(msaa.samples);

        // Queue all entities visible to that view
        for visible_entity in &visible_entities.entities {
            if let Ok((mesh2d_handle, mesh2d_uniform)) = colored_mesh2d.get(*visible_entity) {
                // Get our specialized pipeline
                let mut mesh2d_key = mesh_key;
                if let Some(mesh) = render_meshes.get(&mesh2d_handle.0) {
                    mesh2d_key |=
                        bevy::sprite::Mesh2dPipelineKey::from_primitive_topology(mesh.primitive_topology);
                }

                let pipeline_id =
                    pipelines.specialize(&mut pipeline_cache, &colored_mesh2d_pipeline, mesh2d_key);

                let mesh_z = mesh2d_uniform.transform.w_axis.z;
                transparent_phase.add(bevy::core_pipeline::Transparent2d {
                    entity: *visible_entity,
                    draw_function: draw_colored_mesh2d,
                    pipeline: pipeline_id,
                    // The 2d render items are sorted according to their z value before rendering,
                    // in order to get correct transparency
                    sort_key: bevy::core::FloatOrd(mesh_z),
                    // This material is not batched
                    batch_range: None,
                });
            }
        }
    }
}
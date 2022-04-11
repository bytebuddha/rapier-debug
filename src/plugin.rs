use bevy::prelude::*;
use bevy_render::options::{WgpuFeatures, WgpuOptions};
use bevy_render::render_phase::AddRenderCommand;
use bevy_render::render_resource::SpecializedPipelines;
use bevy_render::{RenderApp, RenderStage};

pub struct RapierDebugPlugin;

impl Plugin for RapierDebugPlugin {
    fn build(&self, app: &mut App) {
        #[cfg(feature = "dim3")]
        plugin_3d(app);
        #[cfg(feature = "dim2")]
        plugin_2d(app);
        app.add_system(crate::systems::spawn_colliders);
    }
}

#[cfg(feature = "dim3")]
fn plugin_3d(app: &mut App) {
    let mut shaders = app.world.get_resource_mut::<Assets<Shader>>().unwrap();
    shaders.set_untracked(
        super::WIREFRAME_SHADER_HANDLE.typed::<Shader>(),
        Shader::from_wgsl(include_str!("shaders/wireframe.wgsl")),
    );

    let mut options = app.world.get_resource_or_insert_with(WgpuOptions::default);

    options.features |= WgpuFeatures::POLYGON_MODE_LINE;

    if let Ok(render_app) = app.get_sub_app_mut(RenderApp) {
        render_app
            .add_render_command::<bevy::core_pipeline::Transparent3d, crate::DrawColliderWireframes>()
            .init_resource::<crate::ColliderWireframePipeline>()
            .init_resource::<SpecializedPipelines<crate::ColliderWireframePipeline>>()
            .add_system_to_stage(RenderStage::Extract, super::systems::extract_wireframes)
            .add_system_to_stage(RenderStage::Queue, super::systems::queue_wireframes);
    }
}

#[cfg(feature = "dim2")]
fn plugin_2d(app: &mut App) {
    // Load our custom shader
    let mut shaders = app.world.get_resource_mut::<Assets<Shader>>().unwrap();
    shaders.set_untracked(
        super::WIREFRAME2D_SHADER_HANDLE.typed::<Shader>(),
        Shader::from_wgsl(include_str!("shaders/wireframe2d.wgsl")),
    );

    let mut options = app.world.get_resource_or_insert_with(WgpuOptions::default);

    options.features |= WgpuFeatures::POLYGON_MODE_LINE;

    // Register our custom draw function and pipeline, and add our render systems
    let render_app = app.get_sub_app_mut(RenderApp).unwrap();
    render_app
        .add_render_command::<bevy::core_pipeline::Transparent2d, super::DrawColliderWireframes2d>()
        .init_resource::<super::ColliderWireframe2dPipeline>()
        .init_resource::<SpecializedPipelines<super::ColliderWireframe2dPipeline>>()
        .add_system_to_stage(RenderStage::Extract, super::systems::extract_wireframes2d)
        .add_system_to_stage(RenderStage::Queue, super::systems::queue_wireframes2d);
}
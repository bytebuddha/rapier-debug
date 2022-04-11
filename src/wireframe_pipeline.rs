use bevy_asset::prelude::*;
use bevy_ecs::prelude::*;
use bevy_pbr::{MeshPipeline, MeshPipelineKey};
use bevy_render::prelude::*;
use bevy_render::render_resource::{
    PolygonMode, RenderPipelineDescriptor, SpecializedPipeline, VertexAttribute,
    VertexBufferLayout, VertexFormat, VertexStepMode,
};
use std::borrow::Cow;

pub struct ColliderWireframePipeline {
    mesh_pipeline: MeshPipeline,
    shader: Handle<Shader>,
}

impl FromWorld for ColliderWireframePipeline {
    fn from_world(world: &mut World) -> ColliderWireframePipeline {
        Self {
            shader: super::WIREFRAME_SHADER_HANDLE.typed(),
            mesh_pipeline: world.get_resource::<MeshPipeline>().unwrap().clone(),
        }
    }
}

impl SpecializedPipeline for ColliderWireframePipeline {
    type Key = MeshPipelineKey;

    fn specialize(&self, key: Self::Key) -> RenderPipelineDescriptor {
        let mut descriptor = self.mesh_pipeline.specialize(key);
        descriptor.label = Some(Cow::Borrowed("rapier_debug_wireframe_pipeline"));
        descriptor.vertex.shader = self.shader.clone_weak();
        descriptor.fragment.as_mut().unwrap().shader = self.shader.clone_weak();
        descriptor.primitive.cull_mode = None;
        descriptor.primitive.polygon_mode = PolygonMode::Line;
        //descriptor.depth_stencil.as_mut().unwrap().bias.slope_scale = 1.0;

        // Barycentric_Position Vec3
        // Vertex_Normal Vec3
        // Vertex_Position Vec3
        // Vertex_Uv Vec2

        let mut attributes = Vec::new();
        let mut cursor = 0;

        // Barycentric_Position
        attributes.push(VertexAttribute {
            format: VertexFormat::Float32x3,
            offset: cursor,
            shader_location: 3,
        });
        cursor += VertexFormat::Float32x3.size();

        // Vertex_Color
        attributes.push(VertexAttribute {
            format: VertexFormat::Float32x3,
            offset: cursor,
            shader_location: 4,
        });
        cursor += VertexFormat::Float32x3.size();

        // Vertex_Dashed
        attributes.push(VertexAttribute {
            format: VertexFormat::Float32x2,
            offset: cursor,
            shader_location: 5,
        });
        cursor += VertexFormat::Float32x2.size();

        // Vertex_Normal
        attributes.push(VertexAttribute {
            format: VertexFormat::Float32x3,
            offset: cursor,
            shader_location: 1,
        });
        cursor += VertexFormat::Float32x3.size();

        // Vertex_Position
        attributes.push(VertexAttribute {
            format: VertexFormat::Float32x3,
            offset: cursor,
            shader_location: 0,
        });
        cursor += VertexFormat::Float32x3.size();

        // Vertex_Uv
        attributes.push(VertexAttribute {
            format: VertexFormat::Float32x2,
            offset: cursor,
            shader_location: 2,
        });
        cursor += VertexFormat::Float32x2.size();

        descriptor.vertex.buffers = vec![VertexBufferLayout {
            array_stride: cursor,
            step_mode: VertexStepMode::Vertex,
            attributes,
        }];

        //dbg!(&descriptor);
        descriptor
    }
}

use std::borrow::Cow;
use bevy::prelude::*;
use bevy::sprite::{Mesh2dPipeline, Mesh2dPipelineKey};
use bevy_render::render_resource::{ PolygonMode, RenderPipelineDescriptor, SpecializedPipeline, VertexAttribute, VertexBufferLayout, VertexFormat, VertexStepMode};

pub struct ColliderWireframe2dPipeline {
    shader: Handle<Shader>,
    mesh2d_pipeline: Mesh2dPipeline
}

impl FromWorld for ColliderWireframe2dPipeline {
    fn from_world(world: &mut World) -> Self {
        Self {
            shader: super::WIREFRAME2D_SHADER_HANDLE.typed(),
            mesh2d_pipeline: Mesh2dPipeline::from_world(world),
        }
    }
}

impl SpecializedPipeline for ColliderWireframe2dPipeline {
    type Key = Mesh2dPipelineKey;

    fn specialize(&self, key: Self::Key) -> RenderPipelineDescriptor {
        let mut descriptor = self.mesh2d_pipeline.specialize(key);
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
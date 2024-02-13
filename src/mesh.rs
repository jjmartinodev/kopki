use bytemuck::{cast_slice, Pod, Zeroable};
use wgpu::util::{BufferInitDescriptor, DeviceExt};

use crate::{render::RenderResource, Context};

pub trait Vertex: Pod + Zeroable {
    fn desc() -> wgpu::VertexBufferLayout<'static>;
}

pub struct StaticMesh {
    vertex_buffer: wgpu::Buffer,
    index_buffer: wgpu::Buffer,
    index_count: u32,
}

impl StaticMesh {
    pub fn new<V:Vertex>(
        ctx: &Context,
        vertices: &[V],
        indices: &[u32]
    ) -> StaticMesh {
        let vertex_buffer = ctx.device.create_buffer_init(&BufferInitDescriptor {
            label: Some("Static Mesh Vertex Buffer"),
            contents: cast_slice(vertices),
            usage: wgpu::BufferUsages::VERTEX
        });

        let index_buffer = ctx.device.create_buffer_init(&BufferInitDescriptor {
            label: Some("Static Mesh Indices Buffer"),
            contents: cast_slice(indices),
            usage: wgpu::BufferUsages::INDEX
        });

        StaticMesh {
            vertex_buffer,
            index_buffer,
            index_count: indices.len() as u32,
        }
    }
    pub fn index_count(&self) -> u32 { self.index_count }
    pub fn vertex_buffer_resource(&self) -> RenderResource<'_> {
        RenderResource::VertexBuffer { slice: self.vertex_buffer.slice(..) }
    }
    pub fn index_buffer_resource(&self) -> RenderResource<'_> {
        RenderResource::IndexBuffer { slice: self.index_buffer.slice(..) }
    }
}
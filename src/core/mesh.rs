use bytemuck::{cast_slice, Pod, Zeroable};
use wgpu::util::{BufferInitDescriptor, DeviceExt};

use super::{context::Context, render::RenderResource};

pub struct StaticMesh {
    vertex_buffer: wgpu::Buffer,
    index_buffer: wgpu::Buffer,
    index_count: u32,
}

impl StaticMesh {
    pub fn new<V: Pod + Zeroable>(
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

pub struct DynamicMesh {
    vertex_buffer: wgpu::Buffer,
    index_buffer: wgpu::Buffer,
    index_count: u32,
}

impl DynamicMesh {
    pub fn new<V: Pod + Zeroable>(
        ctx: &Context,
        vertices: &[V],
        indices: &[u32]
    ) -> DynamicMesh {
        let vertex_buffer = ctx.device.create_buffer_init(&BufferInitDescriptor {
            label: Some("Dynamic Mesh Vertex Buffer"),
            contents: cast_slice(vertices),
            usage: wgpu::BufferUsages::VERTEX | wgpu::BufferUsages::COPY_DST
        });

        let index_buffer = ctx.device.create_buffer_init(&BufferInitDescriptor {
            label: Some("Dynamic Mesh Indices Buffer"),
            contents: cast_slice(indices),
            usage: wgpu::BufferUsages::INDEX | wgpu::BufferUsages::COPY_DST
        });

        DynamicMesh {
            vertex_buffer,
            index_buffer,
            index_count: indices.len() as u32,
        }
    }
    pub fn uptade_vertices<V: Pod + Zeroable>(
        &mut self,
        ctx: &Context,
        vertices: &[V],
    ) {
        ctx.queue.write_buffer(&self.vertex_buffer, 0, cast_slice(vertices));
        todo!("untested, unsafe for now");
    }
    pub fn uptade_indices(
        &mut self,
        ctx: &Context,
        indices: &[u32]
    ) {
        ctx.queue.write_buffer(&self.index_buffer, 0, cast_slice(indices));
        todo!("untested, unsafe for now");
    }
    pub fn index_count(&self) -> u32 { self.index_count }
    pub fn vertex_buffer_resource(&self) -> RenderResource<'_> {
        RenderResource::VertexBuffer { slice: self.vertex_buffer.slice(..) }
    }
    pub fn index_buffer_resource(&self) -> RenderResource<'_> {
        RenderResource::IndexBuffer { slice: self.index_buffer.slice(..) }
    }
}
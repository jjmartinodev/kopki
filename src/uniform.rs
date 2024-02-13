use wgpu::util::{BufferInitDescriptor, DeviceExt};

use crate::Context;

pub struct UniformBuffer {
    buffer: wgpu::Buffer
}

impl UniformBuffer {
    pub fn new(ctx: &Context, data: &[u8]) -> UniformBuffer {
        let buffer = ctx.device.create_buffer_init(&BufferInitDescriptor {
            label: Some("Uniform Buffer"),
            contents: data,
            usage: wgpu::BufferUsages::UNIFORM | wgpu::BufferUsages::COPY_DST
        });

        UniformBuffer { buffer }
    }
    /// uptades data sending a write buffer command throgh the queue
    pub fn uptade(&mut self, ctx: &Context, data: &[u8], offset: u64) {
        ctx.queue.write_buffer(&self.buffer, offset, data)
    }
    pub fn as_resource(&self) -> wgpu::BindingResource {
        self.buffer.as_entire_binding()
    }
    pub fn binding_type() -> wgpu::BindingType {
        wgpu::BindingType::Buffer {
            ty: wgpu::BufferBindingType::Uniform,
            has_dynamic_offset: false,
            min_binding_size: None
        }
    }
}
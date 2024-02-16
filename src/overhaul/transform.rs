
use bytemuck::cast_slice;
use glam::{Mat4, Quat, Vec3};

use crate::core::{context::Context, uniform::UniformBuffer};

use super::OPENGL_TO_WGPU_MATRIX;

pub struct Transform {
    pub translation: Vec3,
    pub rotation: Quat,
    pub scale: Vec3,
    buffer: UniformBuffer
}

impl Transform {
    pub fn new(
        ctx: &Context,
        translation: Vec3,
        rotation: Quat,
        scale: Vec3
    ) -> Transform {
        let transform = Mat4::from_scale_rotation_translation(scale, rotation, translation);
        let data = (OPENGL_TO_WGPU_MATRIX * transform)
            .to_cols_array();
        let buffer = UniformBuffer::new(ctx,cast_slice(&data));
        Transform {
            translation,
            rotation,
            scale,
            buffer,
        }
    }
    pub fn uptade(&mut self, ctx: &Context) {
        let transform = Mat4::from_scale_rotation_translation(
            self.scale,
            self.rotation,
            self.translation
        );

        let data = (OPENGL_TO_WGPU_MATRIX * transform)
            .to_cols_array();
        self.buffer.uptade(ctx, cast_slice(&data), 0);
    }
    pub fn as_binding_resource(&self) -> wgpu::BindingResource {
        self.buffer.as_binding_resource()
    }
    pub fn binding_type() -> wgpu::BindingType {
        UniformBuffer::binding_type()
    }
}
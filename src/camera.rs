use bytemuck::cast_slice;
use glam::{Mat4, Vec3};

use crate::core::{context::Context, uniform::UniformBuffer};

pub enum Projection {
    Orthographic {
        left: f32,
        right: f32,
        bottom: f32,
        top: f32,
        near: f32,
        far:f32
    },
    Perspective {
        z_far: f32,
        z_near: f32,
        aspect_ratio: f32,
        fov_radians: f32
    }
}

pub struct Camera {
    pub position: Vec3,
    pub target: Vec3,
    pub up: Vec3,
    pub projection: Projection,
    pub buffer: UniformBuffer
}

impl Camera {
    pub fn new(
        ctx: &Context,
        position: Vec3,
        target: Vec3,
        up: Vec3,
        projection: Projection
    ) -> Camera {
        let data = Mat4::IDENTITY.to_cols_array();
        let buffer = UniformBuffer::new(ctx, cast_slice(&data));
        Camera {
            position,
            target,
            up,
            projection,
            buffer
        }
    }
    pub fn calculate_projection(&self) -> Mat4 {
        let p = match self.projection {
            Projection::Orthographic
            { left, right, bottom, top, near, far } => {
                Mat4::orthographic_rh(left, right, bottom, top, near, far)
            }
            Projection::Perspective
            { z_far, z_near, aspect_ratio, fov_radians } => {
                Mat4::perspective_rh(fov_radians, aspect_ratio, z_near, z_far)
            }
        };

        let v = Mat4::look_at_rh(self.position, self.target, self.up);

        p * v
    }
    pub fn uptade(&mut self, ctx: &Context) {
        let data = self.calculate_projection().to_cols_array();
        self.buffer.uptade(ctx, cast_slice(&data), 0);
    }
    pub fn as_resource(&self) -> wgpu::BindingResource {
        self.buffer.as_resource()
    }
}
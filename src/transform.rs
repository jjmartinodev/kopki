use crate::OPENGL_TO_WGPU_MATRIX;

pub struct Transform {
    translation: glam::Vec3,
    rotation: (glam::Vec3, f32),
    scale: glam::Vec3
}

impl Transform {
    pub fn build_matrix(&self) -> glam::Mat4 {
        glam::Mat4::from_scale_rotation_translation(
            self.scale,
            glam::Quat::from_axis_angle(self.rotation.0, self.rotation.1),
            self.translation
        )
    }
    pub fn build_wgpu_matrix(&self) -> glam::Mat4 {
        OPENGL_TO_WGPU_MATRIX * self.build_matrix()
    }
}
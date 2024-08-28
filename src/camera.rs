pub enum Projection {
    Ortho {
        left: f32,
        right: f32,
        bottom: f32,
        top: f32,
        near: f32,
        far: f32
    }
}

pub struct Camera {
    projection: Projection,
    eye: glam::Vec3,
    target: glam::Vec3,
    up: glam::Vec3
}
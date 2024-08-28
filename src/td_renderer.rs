pub struct Mesh2DRenderer {
    pipeline: wgpu::RenderPipeline
}

pub struct Mesh2DBuilder {
    material_bind_group_layout: wgpu::BindGroupLayout,
    transform_bind_group_layout: wgpu::BindGroupLayout,
    camera_bind_group_layout: wgpu::BindGroupLayout,
}

pub struct Mesh2D {
    vertex_buffer: wgpu::Buffer,
    index_buffer: wgpu::Buffer
}
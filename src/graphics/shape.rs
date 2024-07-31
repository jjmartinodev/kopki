use std::f32::consts::PI;

use bytemuck::cast_slice;
use wgpu::util::{BufferInitDescriptor, DeviceExt};

use crate::App;

use super::Frame;

const SHAPE_VERTEX_DESC: wgpu::VertexBufferLayout = wgpu::VertexBufferLayout {
    step_mode: wgpu::VertexStepMode::Vertex,
    array_stride: std::mem::size_of::<ShapeVertex>() as u64,
    attributes: &wgpu::vertex_attr_array![0 => Float32x2, 1 => Unorm8x4]
};

#[repr(C)]
#[derive(Debug, Clone, Copy, bytemuck::Zeroable, bytemuck::Pod)]
struct ShapeVertex {
    position: [f32; 2],
    color: [u8; 4]
}

#[derive(Debug, Clone, Copy)]
pub enum Shape {
    Rect {
        x: f32,
        y: f32,
        w: f32,
        h: f32,
        color: [u8;4]
    },
    Circle {
        x: f32,
        y: f32,
        r: f32,
        color: [u8;4]
    },
}

#[repr(C)]
#[derive(Debug, Clone, Copy, bytemuck::Zeroable, bytemuck::Pod)]
struct GlobalUniform {
    framebuffer_size: [f32; 2]
}

pub struct ShapeRenderer {
    pipeline: wgpu::RenderPipeline,
    uniform_buffer: wgpu::Buffer,
    uniform_bind_group: wgpu::BindGroup
}

impl Shape {
    fn vertices(&self) -> (Vec<ShapeVertex>, Vec<u32>) {
        match *self {
            Self::Rect { x, y, w, h, color } => {
                return (
                    vec![
                        ShapeVertex { position: [x,  y],   color },
                        ShapeVertex { position: [x+w,y],   color },
                        ShapeVertex { position: [x+w,y+h], color },
                        ShapeVertex { position: [x,  y+h], color }
                    ],
                    vec![0,1,2,0,2,3]
                );
            }
            Self::Circle { x, y, r, color } => {
                let mut vertices = vec![ShapeVertex {position: [x, y], color}];
                let mut indices = vec![];

                let mut angle = 0.0;
                let mut index = 1;
                let step = 0.2;

                loop {
                    if angle > PI * 2. {
                        break;
                    }
                    angle += step;
                    let next = angle + step;

                    vertices.push(ShapeVertex {position: [angle.cos() * r + x, angle.sin() * r + y], color});
                    vertices.push(ShapeVertex {position: [next.cos() * r + x, next.sin() * r + y], color});
                    indices.push(0);
                    indices.push(index);
                    indices.push(index + 1);
                    index += 2;
                }
                return (vertices, indices)
            }
        }
    }
}

impl ShapeRenderer {
    pub fn new(app: &mut App) -> ShapeRenderer {
        let context = &app.context;
        let window_size = app.window.inner_size();
        let uniform_buffer = context.device.create_buffer_init(&BufferInitDescriptor {
            label: Some("Shape Vertex Buffer"),
            contents: cast_slice(&[GlobalUniform {
                framebuffer_size: [window_size.width as f32, window_size.height as f32]
            }]),
            usage: wgpu::BufferUsages::UNIFORM | wgpu::BufferUsages::COPY_DST
        });
        let uniform_bind_group_layout =
            context.device.create_bind_group_layout(&wgpu::BindGroupLayoutDescriptor {
                entries: &[
                    wgpu::BindGroupLayoutEntry {
                        binding: 0,
                        visibility: wgpu::ShaderStages::VERTEX_FRAGMENT,
                        ty: wgpu::BindingType::Buffer {
                            ty: wgpu::BufferBindingType::Uniform,
                            has_dynamic_offset: false,
                            min_binding_size: None
                        },
                        count: None,
                    },
                ],
                label: Some("shape_uniform_bind_group_layout"),
            });
        
        let uniform_bind_group = context.device.create_bind_group(&wgpu::BindGroupDescriptor {
            label: Some("uniform bind group"),
            layout: &uniform_bind_group_layout,
            entries: &[
                wgpu::BindGroupEntry {
                    binding: 0,
                    resource: uniform_buffer.as_entire_binding()
                }
            ]
        });
        let shader = context.device.create_shader_module(wgpu::include_wgsl!("shape.wgsl"));
        let pipeline_layout =
            context.device.create_pipeline_layout(&wgpu::PipelineLayoutDescriptor {
                label: Some("Shape Render Pipeline Layout"),
                bind_group_layouts: &[
                    &uniform_bind_group_layout
                ],
                push_constant_ranges: &[],
            });

        let pipeline = context.device.create_render_pipeline(&wgpu::RenderPipelineDescriptor {
            label: Some("Shape Render Pipeline"),
            layout: Some(&pipeline_layout),
            vertex: wgpu::VertexState {
                module: &shader,
                entry_point: "vs_main",
                buffers: &[SHAPE_VERTEX_DESC],
                compilation_options: wgpu::PipelineCompilationOptions::default(),
            },
            fragment: Some(wgpu::FragmentState {
                module: &shader,
                entry_point: "fs_main",
                targets: &[Some(wgpu::ColorTargetState {
                    format: app.render_surface.as_ref().unwrap().configuration.format,
                    blend: Some(wgpu::BlendState::REPLACE),
                    write_mask: wgpu::ColorWrites::ALL,
                })],
                compilation_options: wgpu::PipelineCompilationOptions::default(),
            }),
            primitive: wgpu::PrimitiveState {
                topology: wgpu::PrimitiveTopology::TriangleList,
                strip_index_format: None,
                front_face: wgpu::FrontFace::Ccw,
                cull_mode: None,
                polygon_mode: wgpu::PolygonMode::Fill,
                unclipped_depth: false,
                conservative: false,
            },
            depth_stencil: None,
            multisample: wgpu::MultisampleState {
                count: 1,
                mask: !0,
                alpha_to_coverage_enabled: false,
            },
            multiview: None,
            cache: None,
        });

        Self { pipeline, uniform_bind_group, uniform_buffer }
    }
    pub fn render(&self, app: &mut App, frame: &mut Frame, shapes: &[&Shape]) {
        let context = &app.context;

        let size = app.window.inner_size();

        context.queue.write_buffer(&self.uniform_buffer, 0, bytemuck::cast_slice(&[GlobalUniform {
            framebuffer_size: [size.width as f32, size.height as f32]
        }]));

        let mut buffers = vec![];

        for shape in shapes {
            let vertices = shape.vertices();
            buffers.push((
                context.device.create_buffer_init(&BufferInitDescriptor {
                    label: Some("Shape Vertex Buffer"),
                    contents: cast_slice(&vertices.0),
                    usage: wgpu::BufferUsages::VERTEX
                }),
                context.device.create_buffer_init(&BufferInitDescriptor {
                    label: Some("Shape Index Buffer"),
                    contents: cast_slice(&vertices.1),
                    usage: wgpu::BufferUsages::INDEX
                }),
                vertices.1.len() as u32
            ));
        }

        {
            let mut render_pass = frame.encoder.begin_render_pass(&wgpu::RenderPassDescriptor {
                label: Some("Render Pass"),
                color_attachments: &[Some(wgpu::RenderPassColorAttachment {
                    view: &frame.framebuffer_view,
                    resolve_target: None,
                    ops: wgpu::Operations {
                        load: wgpu::LoadOp::Load,
                        store: wgpu::StoreOp::Store,
                    },
                })],
                depth_stencil_attachment: None,
                occlusion_query_set: None,
                timestamp_writes: None,
            });

            render_pass.set_bind_group(0, &self.uniform_bind_group, &[]);
            render_pass.set_pipeline(&self.pipeline);
            for s in buffers.iter().rev() {
                render_pass.set_vertex_buffer(0, s.0.slice(..));
                render_pass.set_index_buffer(s.1.slice(..), wgpu::IndexFormat::Uint32);
                render_pass.draw_indexed(0..s.2 as u32, 0, 0..1);
            }
        }
    }
}

use std::{num::NonZeroU32, ops::Range};

use super::{context::{Context, WindowSurface}, group::GroupLayout};

pub struct Pipeline {
    pipeline: wgpu::RenderPipeline
}

pub enum RenderCommand {
    SetPipeline { resource_index: usize },
    SetBindGroup { index: u32, resource_index: usize },
    SetVertexBuffer { slot: u32, resource_index: usize },
    SetIndexBuffer { resource_index: usize, index_format: wgpu::IndexFormat },
    DrawIndexed { indices: Range<u32>, base_vertex: i32, instances: Range<u32> },
    Draw { vertices: Range<u32>, instances: Range<u32> }
}

pub enum RenderResource<'a> {
    Pipeline { pipeline: &'a wgpu::RenderPipeline },
    VertexBuffer { slice: wgpu::BufferSlice<'a> },
    IndexBuffer { slice: wgpu::BufferSlice<'a> },
    BindGroup { group: &'a wgpu::BindGroup }
}

impl Pipeline {
    /// creates a generic render Pipeline compatible
    /// with the given vertex buffer layouts and bind group layouts
    pub fn new(
        shader_source: wgpu::ShaderModuleDescriptor<'_>,
        ctx: &Context,
        surface: &WindowSurface<'_>,
        vertex_buffer_layouts: &[wgpu::VertexBufferLayout<'static>],
        bind_group_layouts: &[&GroupLayout]
    ) -> Pipeline {
        let shader = ctx.device().create_shader_module(shader_source);

        let pipeline_layout = ctx.device().create_pipeline_layout(
            &wgpu::PipelineLayoutDescriptor {
                label: None,
                bind_group_layouts: bind_group_layouts.iter().map(|e| return &e.layout)
                .collect::<Vec<_>>().as_slice(),
                push_constant_ranges: &[]
            }
        );

        let pipeline = ctx.device().create_render_pipeline(
            &wgpu::RenderPipelineDescriptor {
                label: Some("Render Pipeline"),
                layout: Some(&pipeline_layout),
                vertex: wgpu::VertexState {
                    module: &shader,
                    entry_point: "vs_main",
                    buffers: vertex_buffer_layouts,
                },
                fragment: Some(wgpu::FragmentState {
                    module: &shader,
                    entry_point: "fs_main",
                    targets: &[Some(wgpu::ColorTargetState {
                        format: surface.configuration().format,
                        blend: Some(wgpu::BlendState::REPLACE),
                        write_mask: wgpu::ColorWrites::ALL,
                    })],
                }),
                primitive: wgpu::PrimitiveState {
                    topology: wgpu::PrimitiveTopology::TriangleList,
                    strip_index_format: None,
                    front_face: wgpu::FrontFace::Ccw,
                    cull_mode: Some(wgpu::Face::Back),
                    // Setting this to anything other than Fill requires Features::NON_FILL_POLYGON_MODE
                    polygon_mode: wgpu::PolygonMode::Fill,
                    // Requires Features::DEPTH_CLIP_CONTROL
                    unclipped_depth: false,
                    // Requires Features::CONSERVATIVE_RASTERIZATION
                    conservative: false,
                },
                depth_stencil: None,
                multisample: wgpu::MultisampleState {
                    count: 1,
                    mask: !0,
                    alpha_to_coverage_enabled: false,
                },
                multiview: None,
            }
        );

        Pipeline {
            pipeline
        }
    }
    /// creates a render pipeline with deeper configuration
    pub fn new_extra(
        shader_source: wgpu::ShaderModuleDescriptor<'_>,
        ctx: &Context,
        surface: &WindowSurface<'_>,
        vertex_buffer_layouts: &[wgpu::VertexBufferLayout<'static>],
        bind_group_layouts: &[&GroupLayout],
        primitive: wgpu::PrimitiveState,
        multisample: wgpu::MultisampleState,
        depth_stencil: Option<wgpu::DepthStencilState>,
        multiview: Option<NonZeroU32>
    ) -> Pipeline {
        let shader = ctx.device().create_shader_module(shader_source);

        let pipeline_layout = ctx.device().create_pipeline_layout(
            &wgpu::PipelineLayoutDescriptor {
                label: None,
                bind_group_layouts: bind_group_layouts.iter().map(|e| return &e.layout)
                .collect::<Vec<_>>().as_slice(),
                push_constant_ranges: &[]
            }
        );

        let pipeline = ctx.device().create_render_pipeline(
            &wgpu::RenderPipelineDescriptor {
                label: Some("Render Pipeline"),
                layout: Some(&pipeline_layout),
                vertex: wgpu::VertexState {
                    module: &shader,
                    entry_point: "vs_main",
                    buffers: vertex_buffer_layouts,
                },
                fragment: Some(wgpu::FragmentState {
                    module: &shader,
                    entry_point: "fs_main",
                    targets: &[Some(wgpu::ColorTargetState {
                        format: surface.configuration().format,
                        blend: Some(wgpu::BlendState::REPLACE),
                        write_mask: wgpu::ColorWrites::ALL,
                    })],
                }),
                primitive,
                depth_stencil,
                multisample,
                multiview,
            }
        );

        Pipeline {
            pipeline
        }
    }
    pub fn as_resource(&self) -> RenderResource<'_> {
        RenderResource::Pipeline { pipeline: &self.pipeline }
    }
}
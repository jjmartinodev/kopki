use wgpu::util::{BufferInitDescriptor, DeviceExt};

use crate::App;

pub mod shape;

#[repr(C)]
#[derive(Debug, Clone, Copy, bytemuck::Zeroable, bytemuck::Pod)]
struct UnifromGlobals {
    framebuffer_size: [f32; 2]
}

pub struct Frame {
    present_texture: wgpu::Texture,
    present_sampler: wgpu::Sampler,
    global_uniform_buffer: wgpu::Buffer,
    present_texture_bind_group: wgpu::BindGroup,
    present_texture_bind_group_layout: wgpu::BindGroupLayout,
    global_uniform_bind_group: wgpu::BindGroup,
    present_pipeline: wgpu::RenderPipeline,
}

impl Frame {
    pub(crate) fn new(app: &mut App) -> Frame {
        let context = &app.context;
        let surface = app.render_surface.as_ref().unwrap();

        let texture_size = wgpu::Extent3d {
            width: surface.configuration.width,
            height: surface.configuration.height,
            depth_or_array_layers: 1,
        };

        let present_texture = context.device.create_texture(
            &wgpu::TextureDescriptor {
                size: texture_size,
                mip_level_count: 1,
                sample_count: 1,
                dimension: wgpu::TextureDimension::D2,
                format: surface.format,
                usage: wgpu::TextureUsages::TEXTURE_BINDING | wgpu::TextureUsages::RENDER_ATTACHMENT,
                label: Some("present_texture"),
                view_formats: &[],
            }
        );

        let present_sampler = context.device.create_sampler(&wgpu::SamplerDescriptor {
            address_mode_u: wgpu::AddressMode::ClampToEdge,
            address_mode_v: wgpu::AddressMode::ClampToEdge,
            address_mode_w: wgpu::AddressMode::ClampToEdge,
            mag_filter: wgpu::FilterMode::Linear,
            min_filter: wgpu::FilterMode::Nearest,
            mipmap_filter: wgpu::FilterMode::Nearest,
            ..Default::default()
        });

        let present_texture_view = present_texture.create_view(&wgpu::TextureViewDescriptor::default());

        let global_uniform_buffer = context.device.create_buffer_init(&BufferInitDescriptor {
            label: Some("global_uniform_buffer"),
            contents: &bytemuck::cast_slice(&[
                UnifromGlobals {
                    framebuffer_size: [
                        surface.configuration.width as f32,
                        surface.configuration.height as f32,
                    ]
                }
            ]),
            usage: wgpu::BufferUsages::UNIFORM | wgpu::BufferUsages::COPY_DST
        });

        let present_texture_bind_group_layout =
            context.device.create_bind_group_layout(&wgpu::BindGroupLayoutDescriptor {
                entries: &[
                    wgpu::BindGroupLayoutEntry {
                        binding: 0,
                        visibility: wgpu::ShaderStages::FRAGMENT,
                        ty: wgpu::BindingType::Texture {
                            multisampled: false,
                            view_dimension: wgpu::TextureViewDimension::D2,
                            sample_type: wgpu::TextureSampleType::Float { filterable: true },
                        },
                        count: None,
                    },
                    wgpu::BindGroupLayoutEntry {
                        binding: 1,
                        visibility: wgpu::ShaderStages::FRAGMENT,
                        ty: wgpu::BindingType::Sampler(wgpu::SamplerBindingType::Filtering),
                        count: None,
                    },
                ],
                label: Some("present_texture_bind_group_layout"),
            });

        let globals_bind_group_layout =
            context.device.create_bind_group_layout(&wgpu::BindGroupLayoutDescriptor {
                entries: &[
                    wgpu::BindGroupLayoutEntry {
                        binding: 0,
                        visibility: wgpu::ShaderStages::FRAGMENT,
                        ty: wgpu::BindingType::Buffer {
                            ty: wgpu::BufferBindingType::Uniform, 
                            has_dynamic_offset: false,
                            min_binding_size: None
                        },
                        count: None,
                    },
                ],
                label: Some("present_texture_bind_group_layout"),
            });

        let present_texture_bind_group = context.device.create_bind_group(
            &wgpu::BindGroupDescriptor {
                layout: &present_texture_bind_group_layout,
                entries: &[
                    wgpu::BindGroupEntry {
                        binding: 0,
                        resource: wgpu::BindingResource::TextureView(&present_texture_view),
                    },
                    wgpu::BindGroupEntry {
                        binding: 1,
                        resource: wgpu::BindingResource::Sampler(&present_sampler),
                    }
                ],
                label: Some("present_texture_bind_group"),
            }
        );

        let global_uniform_bind_group = context.device.create_bind_group(
            &wgpu::BindGroupDescriptor {
                layout: &globals_bind_group_layout,
                entries: &[
                    wgpu::BindGroupEntry {
                        binding: 0,
                        resource: wgpu::BindingResource::Buffer(global_uniform_buffer.as_entire_buffer_binding()),
                    },
                ],
                label: Some("global_uniform_bind_group"),
            }
        );

        let present_pipeline_layout = context.device.create_pipeline_layout(
            &wgpu::PipelineLayoutDescriptor {
                label: Some("present_pipeline_layout"),
                bind_group_layouts: &[
                    &present_texture_bind_group_layout,
                    &globals_bind_group_layout
                ],
                push_constant_ranges: &[],
            }
        );

        let present_shader: wgpu::ShaderModule = context.device.create_shader_module(wgpu::include_wgsl!("present.wgsl"));

        let present_pipeline = context.device.create_render_pipeline(&wgpu::RenderPipelineDescriptor {
            label: Some("present_render_pipeline"),
            layout: Some(&present_pipeline_layout),
            vertex: wgpu::VertexState {
                module: &present_shader,
                entry_point: "vs_main",
                buffers: &[],
                compilation_options: Default::default(),
            },
            fragment: Some(wgpu::FragmentState {
                module: &present_shader,
                entry_point: "fs_main",
                targets: &[Some(wgpu::ColorTargetState {
                    format: surface.configuration.format,
                    blend: Some(wgpu::BlendState {
                        color: wgpu::BlendComponent::REPLACE,
                        alpha: wgpu::BlendComponent::REPLACE,
                    }),
                    write_mask: wgpu::ColorWrites::ALL,
                })],
                compilation_options: Default::default(),
            }),
            primitive: wgpu::PrimitiveState {
                topology: wgpu::PrimitiveTopology::TriangleList,
                strip_index_format: None,
                front_face: wgpu::FrontFace::Ccw,
                cull_mode: Some(wgpu::Face::Back),
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

        Frame {
            present_texture,
            present_sampler,
            global_uniform_buffer,
            present_texture_bind_group,
            present_texture_bind_group_layout,
            global_uniform_bind_group,
            present_pipeline
        }
    }
    pub fn present(&self, app: &mut App) {
        app.window.pre_present_notify();

        let context = &app.context;
        let surface = &app.render_surface.as_ref().unwrap().surface;

        let output = surface.get_current_texture().unwrap();
        let view = output
            .texture
            .create_view(&wgpu::TextureViewDescriptor::default());

        let mut encoder = context
            .device
            .create_command_encoder(&wgpu::CommandEncoderDescriptor {
                label: Some("Render Encoder"),
            });

        {
            let mut render_pass = encoder.begin_render_pass(&wgpu::RenderPassDescriptor {
                label: Some("Render Pass"),
                color_attachments: &[Some(wgpu::RenderPassColorAttachment {
                    view: &view,
                    resolve_target: None,
                    ops: wgpu::Operations {
                        load: wgpu::LoadOp::Clear(wgpu::Color { r: 0.0, g: 0.0, b: 0.0, a: 1.0 }),
                        store: wgpu::StoreOp::Store,
                    },
                })],
                depth_stencil_attachment: None,
                occlusion_query_set: None,
                timestamp_writes: None,
            });

            render_pass.set_pipeline(&self.present_pipeline);
            render_pass.set_bind_group(0, &self.present_texture_bind_group, &[]);
            render_pass.set_bind_group(1, &self.global_uniform_bind_group, &[]);
            render_pass.draw(0..3, 0..1);
        }

        context.queue.submit(std::iter::once(encoder.finish()));
        output.present();
    }
    pub fn clear(&mut self, app: &mut App, r: f64, g: f64, b: f64, a: f64) {
        let context = &app.context;

        let view = self
            .present_texture
            .create_view(&wgpu::TextureViewDescriptor::default());

        let mut encoder = context
            .device
            .create_command_encoder(&wgpu::CommandEncoderDescriptor {
                label: Some("Render Encoder"),
            });

        {
            let _render_pass = encoder.begin_render_pass(&wgpu::RenderPassDescriptor {
                label: Some("Render Pass"),
                color_attachments: &[Some(wgpu::RenderPassColorAttachment {
                    view: &view,
                    resolve_target: None,
                    ops: wgpu::Operations {
                        load: wgpu::LoadOp::Clear(wgpu::Color {
                            r,
                            g,
                            b,
                            a
                        }),
                        store: wgpu::StoreOp::Store,
                    },
                })],
                depth_stencil_attachment: None,
                occlusion_query_set: None,
                timestamp_writes: None,
            });
        }

        context.queue.submit(std::iter::once(encoder.finish()));
    }
    pub fn resize(&mut self, app: &mut App) {
        let context = &app.context;
        let surface = app.render_surface.as_ref().unwrap();

        let texture_size = wgpu::Extent3d {
            width: surface.configuration.width,
            height: surface.configuration.height,
            depth_or_array_layers: 1,
        };

        self.present_texture = context.device.create_texture(
            &wgpu::TextureDescriptor {
                size: texture_size,
                mip_level_count: 1,
                sample_count: 1,
                dimension: wgpu::TextureDimension::D2,
                format: surface.format,
                usage: wgpu::TextureUsages::TEXTURE_BINDING | wgpu::TextureUsages::RENDER_ATTACHMENT,
                label: Some("present_texture"),
                view_formats: &[],
            }
        );

        let present_texture_view = self.present_texture.create_view(&wgpu::TextureViewDescriptor::default());

        self.present_texture_bind_group = context.device.create_bind_group(
            &wgpu::BindGroupDescriptor {
                layout: &self.present_texture_bind_group_layout,
                entries: &[
                    wgpu::BindGroupEntry {
                        binding: 0,
                        resource: wgpu::BindingResource::TextureView(&present_texture_view),
                    },
                    wgpu::BindGroupEntry {
                        binding: 1,
                        resource: wgpu::BindingResource::Sampler(&self.present_sampler),
                    }
                ],
                label: Some("present_texture_bind_group"),
            }
        );

        context.queue.write_buffer(&self.global_uniform_buffer, 0, bytemuck::cast_slice(&[UnifromGlobals {
            framebuffer_size: [texture_size.width as f32, texture_size.height as f32]
        }]));
    }
}
pub mod reexports;
pub mod texture;
pub mod transform;
pub mod camera;
pub mod td_renderer;

use std::sync::Arc;

use pollster::FutureExt;
use texture::{RenderableTexture, TextureSampler};
use wgpu::util::{BufferInitDescriptor, DeviceExt};
use winit::window::Window;

#[rustfmt::skip]
pub const OPENGL_TO_WGPU_MATRIX: glam::Mat4 = glam::Mat4::from_cols_array(&[
    1.0, 0.0, 0.0, 0.0,
    0.0, 1.0, 0.0, 0.0,
    0.0, 0.0, 0.5, 0.5,
    0.0, 0.0, 0.0, 1.0,
]);

pub struct RenderInstance {
    pub instance: wgpu::Instance,
}

pub struct RenderDevice {
    pub device: wgpu::Device,
    pub queue: wgpu::Queue,
}

pub type ArcedRenderDevice = Arc<RenderDevice>;

pub struct RenderSurface<'a> {
    pub surface: wgpu::Surface<'a>,
    pub configuration: wgpu::SurfaceConfiguration,
    pub format: wgpu::TextureFormat,
    pub adapter: wgpu::Adapter,
}

pub struct FrameBuffer {
    device: ArcedRenderDevice,
    texture_bind_group_layout: wgpu::BindGroupLayout,
    texture: RenderableTexture,
    sampler: TextureSampler,
    pipeline: wgpu::RenderPipeline,
    texture_bind_group: wgpu::BindGroup,
    global_buffer: wgpu::Buffer,
    global_bind_group: wgpu::BindGroup,
}

impl RenderInstance {
    pub fn new() -> RenderInstance {
        let instance = wgpu::Instance::new(wgpu::InstanceDescriptor {
            backends: wgpu::Backends::all(),
            ..Default::default()
        });

        RenderInstance { instance }
    }

    pub fn surface_from_window<'a>(&self, window: &Arc<Window>) -> RenderSurface<'a> {
        let size = window.inner_size();

        let surface = self.instance.create_surface(window.clone()).unwrap();

        let adapter = self
            .instance
            .request_adapter(&wgpu::RequestAdapterOptionsBase {
                force_fallback_adapter: false,
                compatible_surface: Some(&surface),
                ..Default::default()
            })
            .block_on()
            .unwrap();

        let capabilities = surface.get_capabilities(&adapter);
        let format = capabilities
            .formats
            .iter()
            .find(|f| f.is_srgb())
            .copied()
            .unwrap_or(capabilities.formats[0]);
        let configuration = wgpu::SurfaceConfiguration {
            usage: wgpu::TextureUsages::RENDER_ATTACHMENT,
            format,
            width: size.width,
            height: size.height,
            present_mode: capabilities.present_modes[0],
            alpha_mode: capabilities.alpha_modes[0],
            view_formats: vec![],
            desired_maximum_frame_latency: 2,
        };

        RenderSurface {
            surface,
            configuration,
            format,
            adapter,
        }
    }

    pub fn device_from_instance(&self) -> ArcedRenderDevice {
        let adapter = self
            .instance
            .request_adapter(&wgpu::RequestAdapterOptionsBase {
                force_fallback_adapter: false,
                compatible_surface: None,
                ..Default::default()
            })
            .block_on()
            .unwrap();

        let (device, queue) = adapter
            .request_device(
                &wgpu::DeviceDescriptor {
                    label: Some("Render Device"),
                    required_features: wgpu::Features::empty(),
                    required_limits: wgpu::Limits::default(),
                    ..Default::default()
                },
                None,
            )
            .block_on()
            .unwrap();

        Arc::new(RenderDevice { device, queue })
    }
    pub fn device_from_surface<'a>(
        &self,
        supported_surface: &RenderSurface<'a>,
    ) -> ArcedRenderDevice {
        let (device, queue) = supported_surface
            .adapter
            .request_device(
                &wgpu::DeviceDescriptor {
                    label: Some("Render Device"),
                    required_features: wgpu::Features::empty(),
                    required_limits: wgpu::Limits::default(),
                    ..Default::default()
                },
                None,
            )
            .block_on()
            .unwrap();

        supported_surface
            .surface
            .configure(&device, &supported_surface.configuration);

        Arc::new(RenderDevice { device, queue })
    }
}

impl<'a> RenderSurface<'a> {
    pub fn resize(&mut self, device: &ArcedRenderDevice, width: u32, height: u32) {
        if width == 0 || height == 0 {
            return;
        }
        self.configuration.width = width;
        self.configuration.height = height;
        self.surface.configure(&device.device, &self.configuration);
    }
}

impl FrameBuffer {
    pub fn new(device: &ArcedRenderDevice, surface: &RenderSurface) -> FrameBuffer {
        let global_bind_group_layout =
            device
                .device
                .create_bind_group_layout(&wgpu::BindGroupLayoutDescriptor {
                    label: Some("Texture Bind Group Layout"),
                    entries: &[wgpu::BindGroupLayoutEntry {
                        binding: 0,
                        visibility: wgpu::ShaderStages::FRAGMENT,
                        ty: wgpu::BindingType::Buffer {
                            ty: wgpu::BufferBindingType::Uniform,
                            has_dynamic_offset: false,
                            min_binding_size: None,
                        },
                        count: None,
                    }],
                });
        let texture_bind_group_layout =
            device
                .device
                .create_bind_group_layout(&wgpu::BindGroupLayoutDescriptor {
                    label: Some("Texture Bind Group Layout"),
                    entries: &[
                        wgpu::BindGroupLayoutEntry {
                            binding: 0,
                            visibility: wgpu::ShaderStages::FRAGMENT,
                            ty: wgpu::BindingType::Texture {
                                sample_type: wgpu::TextureSampleType::Float { filterable: true },
                                view_dimension: wgpu::TextureViewDimension::D2,
                                multisampled: false,
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
                });
        let sampler = TextureSampler::new(device);
        let texture = RenderableTexture::from_surface(device, surface);
        let texture_view = texture
            .wgpu_texture()
            .create_view(&wgpu::TextureViewDescriptor {
                label: Some("Texture View"),
                ..Default::default()
            });
        let global_buffer = device.device.create_buffer_init(&BufferInitDescriptor {
            label: Some("Global Uniform Buffer"),
            contents: bytemuck::cast_slice(&[
                surface.configuration.width as f32,
                surface.configuration.height as f32,
            ]),
            usage: wgpu::BufferUsages::UNIFORM | wgpu::BufferUsages::COPY_DST,
        });
        let global_bind_group = device.device.create_bind_group(&wgpu::BindGroupDescriptor {
            label: Some("Global Uniform Bind Group"),
            layout: &global_bind_group_layout,
            entries: &[wgpu::BindGroupEntry {
                binding: 0,
                resource: wgpu::BindingResource::Buffer(global_buffer.as_entire_buffer_binding()),
            }],
        });
        let texture_bind_group = device.device.create_bind_group(&wgpu::BindGroupDescriptor {
            label: Some("Texture Bind Group"),
            layout: &texture_bind_group_layout,
            entries: &[
                wgpu::BindGroupEntry {
                    binding: 0,
                    resource: wgpu::BindingResource::TextureView(&texture_view),
                },
                wgpu::BindGroupEntry {
                    binding: 1,
                    resource: wgpu::BindingResource::Sampler(&sampler.wgpu_sampler()),
                },
            ],
        });
        let pipeline_layout =
            device
                .device
                .create_pipeline_layout(&wgpu::PipelineLayoutDescriptor {
                    label: Some("FrameBuffer Present Pipeline"),
                    bind_group_layouts: &[&texture_bind_group_layout, &global_bind_group_layout],
                    push_constant_ranges: &[],
                });
        let shader = device
            .device
            .create_shader_module(wgpu::include_wgsl!("shaders/present.wgsl"));
        let pipeline = device
            .device
            .create_render_pipeline(&wgpu::RenderPipelineDescriptor {
                label: Some("Render Pipeline"),
                layout: Some(&pipeline_layout),
                vertex: wgpu::VertexState {
                    module: &shader,
                    entry_point: "vs_main",
                    buffers: &[],
                    compilation_options: wgpu::PipelineCompilationOptions::default(),
                },
                fragment: Some(wgpu::FragmentState {
                    module: &shader,
                    entry_point: "fs_main",
                    targets: &[Some(wgpu::ColorTargetState {
                        format: surface.configuration.format,
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

        FrameBuffer {
            device: device.clone(),
            texture_bind_group_layout,
            texture,
            sampler,
            pipeline,
            texture_bind_group,
            global_buffer,
            global_bind_group,
        }
    }
    pub fn present_with_encoder(&self, surface: &RenderSurface, mut encoder: wgpu::CommandEncoder) {
        let output = surface.surface.get_current_texture().unwrap();
        let surface_view = output.texture.create_view(&wgpu::TextureViewDescriptor {
            label: Some("Surface Texture View"),
            ..Default::default()
        });

        {
            let mut render_pass = encoder.begin_render_pass(&wgpu::RenderPassDescriptor {
                label: Some("Present Framebuffer Render Pass"),
                color_attachments: &[Some(wgpu::RenderPassColorAttachment {
                    view: &surface_view,
                    resolve_target: None,
                    ops: wgpu::Operations {
                        load: wgpu::LoadOp::Load,
                        store: wgpu::StoreOp::Store,
                    },
                })],
                depth_stencil_attachment: None,
                timestamp_writes: None,
                occlusion_query_set: None,
            });
            render_pass.set_pipeline(&self.pipeline);
            render_pass.set_bind_group(0, &self.texture_bind_group, &[]);
            render_pass.set_bind_group(1, &self.global_bind_group, &[]);
            render_pass.draw(0..3, 0..1);
        }

        self.device.queue.submit([encoder.finish()]);
        output.present();
    }
    pub fn present(&self, surface: &RenderSurface) {
        let output = surface.surface.get_current_texture().unwrap();
        let surface_view = output.texture.create_view(&wgpu::TextureViewDescriptor {
            label: Some("Surface Texture View"),
            ..Default::default()
        });

        let mut encoder =
            self.device
                .device
                .create_command_encoder(&wgpu::CommandEncoderDescriptor {
                    label: Some("Present Framebuffer Command Encoder"),
                });

        {
            let mut render_pass = encoder.begin_render_pass(&wgpu::RenderPassDescriptor {
                label: Some("Present Framebuffer Render Pass"),
                color_attachments: &[Some(wgpu::RenderPassColorAttachment {
                    view: &surface_view,
                    resolve_target: None,
                    ops: wgpu::Operations {
                        load: wgpu::LoadOp::Load,
                        store: wgpu::StoreOp::Store,
                    },
                })],
                depth_stencil_attachment: None,
                timestamp_writes: None,
                occlusion_query_set: None,
            });
            render_pass.set_pipeline(&self.pipeline);
            render_pass.set_bind_group(0, &self.texture_bind_group, &[]);
            render_pass.set_bind_group(1, &self.global_bind_group, &[]);
            render_pass.draw(0..3, 0..1);
        }

        self.device.queue.submit([encoder.finish()]);
        output.present();
    }
    pub fn rebuild(&mut self, surface: &RenderSurface) {
        self.texture = RenderableTexture::from_surface(&self.device, surface);
        let texture_view = self
            .texture
            .wgpu_texture()
            .create_view(&wgpu::TextureViewDescriptor {
                label: Some("Texture View"),
                ..Default::default()
            });
        self.texture_bind_group =
            self.device
                .device
                .create_bind_group(&wgpu::BindGroupDescriptor {
                    label: Some("Texture Bind Group"),
                    layout: &self.texture_bind_group_layout,
                    entries: &[
                        wgpu::BindGroupEntry {
                            binding: 0,
                            resource: wgpu::BindingResource::TextureView(&texture_view),
                        },
                        wgpu::BindGroupEntry {
                            binding: 1,
                            resource: wgpu::BindingResource::Sampler(&self.sampler.wgpu_sampler()),
                        },
                    ],
                });

        self.device.queue.write_buffer(&self.global_buffer, 0, bytemuck::cast_slice(&[
            surface.configuration.width as f32,
            surface.configuration.height as f32
        ]));
    }
    pub const fn renderable_texture(&self) -> &RenderableTexture {
        &self.texture
    }
}

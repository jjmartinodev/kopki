pub use wgpu;
pub use winit;
use winit::dpi::PhysicalSize;

/// Represents a graphical context to access the gpu through backends.
pub struct Context {
    instance: wgpu::Instance,
    adapter: wgpu::Adapter,
    device: wgpu::Device,
    queue: wgpu::Queue,
}

/// Represents a surface of a window for rendering.
///
/// # Safety:
/// The window a surface targets has to remain valid,
/// for the entirety of the surface's lifetime.
pub struct WindowSurface<'a> {
    surface: wgpu::Surface<'a>,

    configuration: wgpu::SurfaceConfiguration,
    capabilities: wgpu::SurfaceCapabilities,
    format: wgpu::TextureFormat
}

/// A pipeline that intakes ```PipelineResource```s, to draw a
/// scene to a ```WindowSurface``` with a ```Context```.
pub struct Pipeline {
    pipeline: wgpu::RenderPipeline
}

pub enum PipelineResource<'a> {
    VertexBuffer(wgpu::BufferSlice<'a>),
    IndexBuffer(wgpu::BufferSlice<'a>),
}

impl Context {
    /// Creates a context blocking the thread.
    pub fn blocked_new() -> Context {
        pollster::block_on(Context::new())
    }
    /// Creates a context asynchronously.
    pub async fn new() -> Context {
        let instance = wgpu::Instance::new(wgpu::InstanceDescriptor {
            backends: wgpu::Backends::all(),
            ..Default::default()
        });

        let adapter = instance.request_adapter(&wgpu::RequestAdapterOptionsBase {
            power_preference: wgpu::PowerPreference::None,
            force_fallback_adapter: false,
            compatible_surface: None
        }).await.expect("failed to create a context adapter");

        let (device, queue) = adapter.request_device(
            &wgpu::DeviceDescriptor { 
                label: None,
                required_features: wgpu::Features::empty(),
                required_limits: wgpu::Limits::default()
            },
            None
        ).await.expect("failed to create a context device");
    
        Context {
            instance,
            adapter,
            device,
            queue
        }
    }
    /// create a window surface with a context
    pub fn create_surface<'a>(&self, window:&'a winit::window::Window) -> WindowSurface<'a> {
        let size = window.inner_size();

        let surface = unsafe {
            self.instance.create_surface_unsafe(
                wgpu::SurfaceTargetUnsafe::from_window(&window).unwrap()
            ).expect("failed to create window surface")
        };

        let capabilities = surface.get_capabilities(&self.adapter);
        let format = capabilities.formats.iter()
            .copied()
            .filter(|f| f.is_srgb())
            .next()
            .unwrap_or(capabilities.formats[0]);
        let configuration = wgpu::SurfaceConfiguration {
            usage: wgpu::TextureUsages::RENDER_ATTACHMENT,
            format: format,
            width: size.width,
            height: size.height,
            present_mode: capabilities.present_modes[0],
            alpha_mode: capabilities.alpha_modes[0],
            view_formats: vec![],
            desired_maximum_frame_latency: 0
        };
        surface.configure(&self.device, &configuration);
        WindowSurface {
            surface,

            configuration,
            capabilities,
            format
        }
    }
    pub fn render(
        &self,
        surface: &WindowSurface,
        pipelines: &[Pipeline]
    ) {
        let output = surface.surface.get_current_texture().unwrap();
        let view = output.texture.create_view(&wgpu::TextureViewDescriptor {
            label: Some("Surface Output Texture View"),
            ..Default::default()
        });
        let mut encoder = self.device.create_command_encoder(
    &wgpu::CommandEncoderDescriptor {
            label: Some("Render Encoder"),
        });

        {
            let mut render_pass = encoder.begin_render_pass(
                &wgpu::RenderPassDescriptor {
                label: Some("Render Pass"),
                color_attachments: &[Some(wgpu::RenderPassColorAttachment {
                    view: &view,
                    resolve_target: None,
                    ops: wgpu::Operations {
                        load: wgpu::LoadOp::Clear(wgpu::Color {
                            r: 0.1,
                            g: 0.2,
                            b: 0.3,
                            a: 1.0,
                        }),
                        store: wgpu::StoreOp::Store,
                    },
                })],
                depth_stencil_attachment: None,
                occlusion_query_set: None,
                timestamp_writes: None,
            });

            for pipeline in pipelines {
                render_pass.set_pipeline(&pipeline.pipeline)
            }
        }
    
        self.queue.submit(std::iter::once(encoder.finish()));
        output.present();    
    }
}

impl<'a> WindowSurface<'a> {
    /// resize
    pub fn resize(&mut self, ctx: &Context, size: PhysicalSize<u32>) {
        if size.width > 0 && size.height > 0 {
            self.configuration.width = size.width;
            self.configuration.height = size.height;
            self.surface.configure(&ctx.device, &self.configuration);
        }
    }
}

impl Pipeline {
    pub fn new(
        shader_source: wgpu::ShaderSource<'_>,
        ctx: &Context,
        surface: &WindowSurface<'_>
    ) -> Pipeline {
        let shader = ctx.device.create_shader_module(
            wgpu::ShaderModuleDescriptor {
                label: Some("Render Pipeline Shader Module"),
                source: shader_source
            }
        );

        let pipeline_layout = ctx.device.create_pipeline_layout(
            &wgpu::PipelineLayoutDescriptor {
                label: None,
                bind_group_layouts: &[],
                push_constant_ranges: &[]
            }
        );

        let pipeline = ctx.device.create_render_pipeline(
            &wgpu::RenderPipelineDescriptor {
                label: Some("Render Pipeline"),
                layout: Some(&pipeline_layout),
                vertex: wgpu::VertexState {
                    module: &shader,
                    entry_point: "vs_main",
                    buffers: &[],
                },
                fragment: Some(wgpu::FragmentState {
                    module: &shader,
                    entry_point: "fs_main",
                    targets: &[Some(wgpu::ColorTargetState {
                        format: surface.configuration.format,
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
}
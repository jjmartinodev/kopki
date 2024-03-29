use super::render::{RenderCommand, RenderGroup, RenderResource};
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
    pub fn device(&self) -> &wgpu::Device {
        &self.device
    }
    pub fn queue(&self) -> &wgpu::Queue {
        &self.queue
    }
    pub fn instance(&self) -> &wgpu::Instance {
        &self.instance
    }
    pub fn adapter(&self) -> &wgpu::Adapter {
        &self.adapter
    }
    pub fn create_surface<'a>(&self, window:&winit::window::Window) -> WindowSurface<'a> {
        let size = window.inner_size();

        let surface = unsafe {
            self.instance.create_surface_unsafe(
                wgpu::SurfaceTargetUnsafe::from_window(&window).unwrap()
            ).expect("failed to create window surface")
        };

        let capabilities = surface.get_capabilities(&self.adapter);
        let format = capabilities.formats.iter()
            .copied()
            .find(|f| f.is_srgb())
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
    // creates a renders to a surface using a render pass, commands and resources.
    pub fn render<'a>(
        &self,
        surface: &WindowSurface,
        render_groups: &[&RenderGroup],
        color: wgpu::Color
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
                        load: wgpu::LoadOp::Clear(color),
                        store: wgpu::StoreOp::Store,
                    },
                })],
                depth_stencil_attachment: None,
                occlusion_query_set: None,
                timestamp_writes: None,
            });

            for group in render_groups {
                for command in group.commands {
                    match command {
                        RenderCommand::SetPipeline { resource_index } => {
                            match group.resources[*resource_index] {
                                RenderResource::Pipeline { pipeline } => {
                                    render_pass.set_pipeline(pipeline)
                                }
                                _ => panic!()
                            }
                        }
                        RenderCommand::SetVertexBuffer { slot, resource_index } => {
                            match group.resources[*resource_index] {
                                RenderResource::VertexBuffer { slice } => {
                                    render_pass.set_vertex_buffer(*slot, slice)
                                }
                                _ => panic!()
                            }
                        }
                        RenderCommand::SetIndexBuffer {
                            resource_index,
                            index_format 
                        } => {
                            match group.resources[*resource_index] {
                                RenderResource::IndexBuffer { slice } => {
                                    render_pass.set_index_buffer(slice, *index_format)
                                }
                                _ => panic!()
                            }
                        }
                        RenderCommand::DrawIndexed {
                            indices,
                            base_vertex,
                            instances
                        } => {
                            render_pass.draw_indexed(indices.clone(),* base_vertex, instances.clone())
                        }
                        RenderCommand::Draw {
                            vertices,
                            instances
                        } => {
                            render_pass.draw(vertices.clone(), instances.clone())
                        }
                        RenderCommand::SetBindGroup { index, resource_index } => {
                            match group.resources[*resource_index] {
                                RenderResource::BindGroup { group } => {
                                    render_pass.set_bind_group(
                                        *index,
                                        group,
                                        &[]
                                    )
                                }
                                _ => panic!()
                            }
                        }
                    }
                }
            }
        }
    
        self.queue.submit(std::iter::once(encoder.finish()));
        output.present();    
    }
}

impl<'a> WindowSurface<'a> {
    /// resizes the surface of the targetted window
    ///
    /// Safety: if the window and it's surface aren't the same size Unkown Behavior is expected
    pub fn resize(&mut self, ctx: &Context, size: PhysicalSize<u32>) {
        if size.width > 0 && size.height > 0 {
            self.configuration.width = size.width;
            self.configuration.height = size.height;
            self.surface.configure(&ctx.device, &self.configuration);
        }
    }
    pub fn configuration(&self) -> &wgpu::SurfaceConfiguration {
        &self.configuration
    }
    pub fn capabilities(&self) -> &wgpu::SurfaceCapabilities {
        &self.capabilities
    }
    pub fn format(&self) -> &wgpu::TextureFormat {
        &self.format
    }
}
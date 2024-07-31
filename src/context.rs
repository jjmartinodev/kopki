use std::sync::Arc;

use winit::{event_loop::EventLoop, window::{Window, WindowBuilder}};

pub struct Context {
    pub instance: wgpu::Instance,
    pub adapter: wgpu::Adapter,
    pub device: wgpu::Device,
    pub queue: wgpu::Queue
}

pub struct RenderSurface<'a> {
    pub surface: wgpu::Surface<'a>,

    pub configuration: wgpu::SurfaceConfiguration,
    pub capabilities: wgpu::SurfaceCapabilities,
    pub format: wgpu::TextureFormat
}

impl Context {
    pub async fn new(event_loop: &EventLoop<()>) -> Context {
        let instance = wgpu::Instance::new(wgpu::InstanceDescriptor {
            backends: wgpu::Backends::PRIMARY,
            ..Default::default()
        });

        let dummy_window = WindowBuilder::new()
        .with_visible(false)
        .build(&event_loop)
        .unwrap();

        let dummy_surface = instance.create_surface(&dummy_window).unwrap();

        let adapter = instance.request_adapter(
            &wgpu::RequestAdapterOptions {
                power_preference: wgpu::PowerPreference::default(),
                compatible_surface: Some(&dummy_surface),
                force_fallback_adapter: false,
            },
        ).await.unwrap();

        drop(dummy_surface);
        drop(dummy_window);

        let (device, queue) = adapter.request_device(
            &wgpu::DeviceDescriptor {
                required_features: wgpu::Features::empty(),
                required_limits:  wgpu::Limits::default(),
                label: None,
                memory_hints: wgpu::MemoryHints::Performance
            },
            None, // Trace path
        ).await.unwrap();

        Context {
            instance,
            adapter,
            device,
            queue
        }
    }
    pub fn resize_surface(&mut self, new_size: winit::dpi::PhysicalSize<u32>, surface: &mut RenderSurface<'_>) {
        if new_size.width > 0 && new_size.height > 0 {
            surface.configuration.width = new_size.width;
            surface.configuration.height = new_size.height;
            surface.surface.configure(&self.device, &surface.configuration);
        }
    }
    pub fn device(&self) -> &wgpu::Device  { &self.device }
    pub fn queue(&self) -> &wgpu::Queue  { &self.queue }
}

impl<'a> RenderSurface<'a> {
    pub fn from_window(window: Arc<Window>, context: &Context) -> RenderSurface<'a> {
        let size = window.inner_size();
        let surface = context.instance.create_surface(window.clone()).unwrap();

        let capabilities = surface.get_capabilities(&context.adapter);

        let format = capabilities.formats.iter()
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
            capabilities,
            configuration,
            format
        }
    }
    pub fn surface(&self) -> &wgpu::Surface { &self.surface }
}
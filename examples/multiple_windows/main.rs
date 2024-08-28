use std::collections::HashMap;
use std::sync::Arc;

use kopki::reexports::{wgpu, winit};
use winit::{error::EventLoopError, event_loop::EventLoop, window::WindowBuilder};
use kopki::{FrameBuffer, RenderSurface};
use kopki::RenderInstance;

pub struct Window<'a> {
    window: Arc<winit::window::Window>,
    surface: RenderSurface<'a>,
    framebuffer: FrameBuffer
}

fn main() -> Result<(), EventLoopError> {
    let event_loop = EventLoop::new().unwrap();
    let render_instance = RenderInstance::new();
    let render_device;

    let builder = WindowBuilder::new();
    let mut windows = HashMap::new();
    {
        let window = Arc::new(builder.clone().build(&event_loop).unwrap());
        let surface = render_instance.surface_from_window(&window);
        render_device = render_instance.device_from_surface(&surface);
        let framebuffer = FrameBuffer::new(&render_device, &surface);
        windows.insert(window.id(), Window {
            window,
            surface,
            framebuffer
        });
    }
    for _ in 0..3  {
        let window = Arc::new(builder.clone().build(&event_loop).unwrap());
        let surface = render_instance.surface_from_window(&window);
        let framebuffer = FrameBuffer::new(&render_device, &surface);
        windows.insert(window.id(), Window {
            window,
            surface: surface,
            framebuffer
        });
    }

    use winit::event::{Event, WindowEvent};
    event_loop.run(|event, elwt| match event {
        Event::AboutToWait => {
            if windows.is_empty() {
                elwt.exit();
            }

            for window in windows.values() {
                window.window.request_redraw();
            }
        }
        Event::WindowEvent { window_id, event } => match event {
            WindowEvent::RedrawRequested => {
                let window = &windows[&window_id];

                window.window.pre_present_notify();
                let mut encoder =
                    render_device
                        .device
                        .create_command_encoder(&wgpu::CommandEncoderDescriptor {
                            label: Some("Render Encoder"),
                        });
                window.framebuffer.renderable_texture().clear_pass_with_encoder(
                    &mut encoder,
                    0.0,
                    1.0,
                    1.0,
                    0.0,
                );
                window.framebuffer.present_with_encoder(&window.surface, encoder);
            }
            WindowEvent::CloseRequested => {
                windows.remove(&window_id);
            }
            WindowEvent::Resized(size) => {
                let window = windows.get_mut(&window_id).unwrap();
                window.surface.resize(&render_device, size.width, size.height);
                window.framebuffer.rebuild(&window.surface);
            }
            _ => ()
        },
        Event::LoopExiting => {
            
        }
        _ => (),
    })?;

    Ok(())
}

use std::sync::Arc;

use kopki::reexports::winit::{error::EventLoopError, event_loop::EventLoop, window::Window};
use kopki::FrameBuffer;
use kopki::RenderInstance;

fn main() -> Result<(), EventLoopError> {
    let event_loop = EventLoop::new().unwrap();
    let window = Arc::new(Window::new(&event_loop).unwrap());

    let render_instance = RenderInstance::new();
    let mut render_surface = Some(render_instance.surface_from_window(&window));
    let render_device = render_instance.device_from_surface(&render_surface.as_ref().unwrap());
    let mut framebuffer = FrameBuffer::new(&render_device, &render_surface.as_ref().unwrap());

    use winit::event::{Event, WindowEvent};
    event_loop.run(|event, elwt| match event {
        Event::AboutToWait => {
            window.request_redraw();
        }
        Event::WindowEvent { event, .. } => match event {
            WindowEvent::RedrawRequested => {
                window.pre_present_notify();
                let mut encoder =
                    render_device
                        .device
                        .create_command_encoder(&wgpu::CommandEncoderDescriptor {
                            label: Some("Render Encoder"),
                        });
                framebuffer.renderable_texture().clear_pass_with_encoder(
                    &mut encoder,
                    0.0,
                    1.0,
                    1.0,
                    0.0,
                );
                framebuffer.present_with_encoder(render_surface.as_ref().unwrap(), encoder);
            }
            WindowEvent::Resized(size) => {
                render_surface
                    .as_mut()
                    .unwrap()
                    .resize(&render_device, size.width, size.height);
                framebuffer.rebuild(&render_surface.as_ref().unwrap());
            }
            WindowEvent::CloseRequested => elwt.exit(),
            _ => (),
        },
        Event::LoopExiting => {
            render_surface = None;
        }
        _ => (),
    })?;

    Ok(())
}

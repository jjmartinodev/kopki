# kopki
    Simple graphical engine, based on wgpu and winit.

# Minimal Example
    ```
    use kopki::{
        wgpu, winit, Context
    };
    use winit::{
        event::{Event, WindowEvent},
        event_loop::EventLoop,
        window::WindowBuilder
    };

    fn main() {
        let ctx = Context::blocked_new();
        let event_loop = EventLoop::new().unwrap();
        let window = WindowBuilder::new()
        .build(&event_loop).expect("failed to build");
        let mut surface = ctx.create_surface(&window);

        _ = event_loop.run(move |event,elwt| {
            match event {
                Event::AboutToWait => {

                }
                Event::WindowEvent { event, .. } => {
                    match event {
                        WindowEvent::CloseRequested => elwt.exit(),
                        WindowEvent::RedrawRequested => {
                            ctx.render(
                                &surface,&[],&[],
                                wgpu::Color {
                                    r: 0.0,
                                    g: 0.0,
                                    b: 0.0,
                                    a: 0.0,
                                }
                            )
                        }
                        WindowEvent::Resized(size) => {
                            surface.resize(&ctx, size)
                        }
                        _ => ()
                    }
                }
                _ => ()
            }
        })
    }
    ```
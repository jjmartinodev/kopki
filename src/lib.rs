use std::sync::Arc;

use context::{Context, RenderSurface};
use graphics::{Frame, FrameBuffer};
use winit::{event::{Event, WindowEvent}, event_loop::EventLoop, window::{Window, WindowBuilder}};

mod context;
pub mod graphics;

pub trait AppState {
    fn start(app: &mut App) -> Self;
    fn uptade(&mut self, app: &mut App, frame: Frame);
}

pub struct App<'a> {
    event_loop: Option<EventLoop<()>>,
    window: Arc<Window>,
    context: Context,
    render_surface: Option<RenderSurface<'a>>
}

impl<'a> App<'a> {
    pub fn wgpu_surface(&self) -> &RenderSurface {
        &self.render_surface.as_ref()
            .expect("no surface avalible")
    }
    pub fn wgpu_context(&self) -> &Context {
        &self.context
    }
    pub fn new() -> App<'a> {
        let event_loop = EventLoop::new().unwrap();
        let window = Arc::new(
            WindowBuilder::new()
            .with_resizable(false)
            .build(&event_loop)
            .unwrap()
        );
        let context = pollster::block_on(Context::new(&event_loop));
        let render_surface = RenderSurface::from_window(window.clone(), &context);
        App {
            event_loop: Some(event_loop),
            window,
            context,
            render_surface: Some(render_surface)
        }
    }
    pub fn run<State: AppState>(mut self) {
        let mut state = State::start(&mut self);
        let mut framebuffer = FrameBuffer::new(&mut self); 

        _ = self.event_loop.take().unwrap().run(move |event, elwt| match event {
            Event::AboutToWait => {
                self.window.request_redraw();
            }
            Event::WindowEvent { event, .. } => match event {
                WindowEvent::CloseRequested => elwt.exit(),
                WindowEvent::Resized(new_size) => {
                    self.context.resize_surface(new_size, &mut self.render_surface.as_mut().unwrap());
                    framebuffer.resize(&mut self);
                }
                WindowEvent::RedrawRequested => {
                    let frame = framebuffer.frame(&self);
                    state.uptade(&mut self, frame);
                    framebuffer.present(&mut self);
                }
                _ => ()
            }
            Event::LoopExiting => {
                self.render_surface = None;
            }
            _ => ()
        });
    }
}
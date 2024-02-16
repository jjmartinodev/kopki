use std::mem;

use bytemuck::{Pod, Zeroable};
use kopki::{
    bytemuck,
    core::{
        context::Context,
        mesh::DynamicMesh,
        render::{
            Pipeline,
            RenderCommand
        }
    },
    wgpu,
    winit
};
use winit::{
    event::{Event, WindowEvent},
    event_loop::EventLoop,
    window::WindowBuilder
};

#[repr(C)]
#[derive(Debug, Clone, Copy, Pod, Zeroable)]
struct Vertex {
    position: [f32; 2],
    color: [u8; 4],
}

const VERTEX_LAYOUT: wgpu::VertexBufferLayout<'static> = wgpu::VertexBufferLayout {
    array_stride: mem::size_of::<Vertex>() as u64,
    step_mode: wgpu::VertexStepMode::Vertex,
    attributes: &wgpu::vertex_attr_array![0 => Float32x2, 1 => Unorm8x4]
};

const INDICES: [u32; 3] = [0,1,2];

fn main() {
    let mut vertices = [
        Vertex { position: [-1.,-1.], color: [255,0,0,255] },
        Vertex { position: [ 1.,-1.], color: [0,255,0,255] },
        Vertex { position: [ 0., 1.], color: [0,0,255,255] },
    ];

    let ctx = Context::blocked_new();
    let event_loop = EventLoop::new().unwrap();
    let window = WindowBuilder::new()
    .build(&event_loop).expect("failed to build");
    let mut surface = ctx.create_surface(&window);
    let pipeline = Pipeline::new(
        wgpu::include_wgsl!("shader.wgsl"),
        &ctx,
        &surface,
        &[VERTEX_LAYOUT],
        &[]
    );
    let mut mesh = DynamicMesh::new(&ctx, &vertices, &INDICES);
    let mut t = 0.0f32;

    _ = event_loop.run(move |event,elwt| {
        match event {
            Event::AboutToWait => {
                window.request_redraw();
                t += 0.001;
                vertices[0].position[0] = t.sin();
                vertices[0].position[1] = t.cos();
                mesh.uptade_vertices(&ctx, &vertices);
            }
            Event::WindowEvent { event, .. } => {
                match event {
                    WindowEvent::CloseRequested => elwt.exit(),
                    WindowEvent::RedrawRequested => {
                        ctx.render(
                            &surface,
                            &[
                                RenderCommand::SetPipeline { resource_index: 0 },
                                RenderCommand::SetVertexBuffer
                                { slot: 0, resource_index: 1 },
                                RenderCommand::SetIndexBuffer
                                { resource_index: 2, index_format: wgpu::IndexFormat::Uint32 },
                                RenderCommand::DrawIndexed
                                { indices: 0..3, base_vertex: 0, instances: 0..1 } 
                            ],
                            &[
                                pipeline.as_resource(),
                                mesh.vertex_buffer_resource(),
                                mesh.index_buffer_resource()
                            ],
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
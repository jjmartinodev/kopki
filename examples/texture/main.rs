use std::mem;

use kopki::{
    bytemuck,
    core::{
        context::Context,
        group::{
            Group,
            GroupLayout
        },
        mesh::StaticMesh,
        render::{
            Pipeline,
            RenderCommand
        },
        texture::{
            Sampler2D,
            Texture2D
        }
    },
    wgpu,
    winit
};
use bytemuck::{Pod, Zeroable};
use winit::{
    event::{Event, WindowEvent},
    event_loop::EventLoop,
    window::WindowBuilder
};

#[repr(C)]
#[derive(Debug, Clone, Copy, Pod, Zeroable)]
struct Vertex {
    position: [f32; 2],
    uv: [f32; 2],
}

const VERTEX_LAYOUT: wgpu::VertexBufferLayout<'static> = wgpu::VertexBufferLayout {
    array_stride: mem::size_of::<Vertex>() as u64,
    step_mode: wgpu::VertexStepMode::Vertex,
    attributes: &wgpu::vertex_attr_array![0 => Float32x2, 1 => Float32x2]
};

const VERTICES: [Vertex; 3] = [
    Vertex { position: [-1.,-1.], uv: [0.,0.] },
    Vertex { position: [ 1.,-1.], uv: [1.,0.] },
    Vertex { position: [ 0., 1.], uv: [1.,1.] },
];

const INDICES: [u32; 3] = [0,1,2];

fn main() {
    let ctx = Context::blocked_new();
    let event_loop = EventLoop::new().unwrap();
    let window = WindowBuilder::new()
    .build(&event_loop).expect("failed to build");
    let mut surface = ctx.create_surface(&window);
    let group_layout = GroupLayout::new(&ctx, &[
        wgpu::BindGroupLayoutEntry {
            binding: 0,
            ty: Texture2D::binding_type(),
            visibility: wgpu::ShaderStages::FRAGMENT,
            count: None
        },
        wgpu::BindGroupLayoutEntry {
            binding: 1,
            ty: Sampler2D::binding_type(),
            visibility: wgpu::ShaderStages::FRAGMENT,
            count: None
        }
    ]);
    let data = [255,0,0,0,  0,255,0,0, 0,0,255,0,  255,0,255,0];
    let texture = Texture2D::new(
        &ctx, &data, wgpu::TextureFormat::Rgba8Unorm, 2, 2);
    let sampler = Sampler2D::default(&ctx);
    let pipeline = Pipeline::new(
        wgpu::include_wgsl!("shader.wgsl"),
        &ctx,
        &surface,
        &[VERTEX_LAYOUT],
        &[&group_layout]
    );
    let group = Group::new(&ctx, &group_layout, vec![
        texture.as_bind_resource(),
        sampler.as_bind_resource()
    ]);
    let mesh = StaticMesh::new(&ctx, &VERTICES, &INDICES);

    _ = event_loop.run(move |event,elwt| {
        match event {
            Event::AboutToWait => {

            }
            Event::WindowEvent { event, .. } => {
                match event {
                    WindowEvent::CloseRequested => elwt.exit(),
                    WindowEvent::RedrawRequested => {
                        ctx.render(
                            &surface,
                            &[
                                RenderCommand::SetPipeline { resource_index: 0 },
                                RenderCommand::SetBindGroup
                                { index: 0, resource_index: 3 },
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
                                mesh.index_buffer_resource(),
                                group.as_resource()
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
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
            RenderCommand, RenderGroup
        },
        texture::{
            Sampler2D,
            Texture2D,
            TextureView2D
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

const VERTICES: [Vertex; 4] = [
    Vertex { position: [-1.,-1.], uv: [-1.,-1.] },
    Vertex { position: [ 1.,-1.], uv: [ 1.,-1.] },
    Vertex { position: [ 1., 1.], uv: [ 1., 1.] },
    Vertex { position: [-1., 1.], uv: [-1., 1.] },
];

const INDICES: [u32; 6] = [0,1,2, 3,2,0];

fn main() {
    let ctx = Context::blocked_new();
    let event_loop = EventLoop::new().unwrap();
    let window = WindowBuilder::new()
    .build(&event_loop).expect("failed to build");
    let mut surface = ctx.create_surface(&window);
    let group_layout = GroupLayout::new(&ctx, &[
        wgpu::BindGroupLayoutEntry {
            binding: 0,
            ty: TextureView2D::binding_type(),
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
        &ctx, &data, wgpu::TextureFormat::Rgba8Unorm, 2, 2, 1);
    let view = texture.create_view();
    let sampler = Sampler2D::from_descriptor(&ctx, &wgpu::SamplerDescriptor {
        label: None, 
        address_mode_u: wgpu::AddressMode::MirrorRepeat,
        address_mode_v: wgpu::AddressMode::MirrorRepeat,
        address_mode_w: wgpu::AddressMode::MirrorRepeat,
        mag_filter: wgpu::FilterMode::Nearest,
        min_filter: wgpu::FilterMode::Nearest,
        ..Default::default()
    });
    let pipeline = Pipeline::new(
        wgpu::include_wgsl!("shader.wgsl"),
        &ctx,
        &surface,
        &[VERTEX_LAYOUT],
        &[&group_layout]
    );
    let group = Group::new(&ctx, &group_layout, vec![
        view.as_bind_resource(),
        sampler.as_bind_resource()
    ]);
    let mesh = StaticMesh::new(&ctx, &VERTICES, &INDICES);
    
    _ = event_loop.run(move |event,elwt| {
        match event {
            Event::AboutToWait => {
                window.request_redraw();
            }
            Event::WindowEvent { event, .. } => {
                match event {
                    WindowEvent::CloseRequested => elwt.exit(),
                    WindowEvent::RedrawRequested => {
                        let renderer = RenderGroup {
                            commands: &[
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
                            resources: &[
                                pipeline.as_resource(),
                                mesh.vertex_buffer_resource(),
                                mesh.index_buffer_resource(),
                                group.as_resource()
                            ]
                        };
                        ctx.render(
                            &surface,
                            &[&renderer],
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
use kopki::{
    graphics::Frame, App, AppState
};

struct State {
    pipeline: wgpu::RenderPipeline
}

impl AppState for State {
    fn start(app: &mut App) -> Self {
        let context = app.wgpu_context();
        let surface = app.wgpu_surface();

        let pipeline_layout = context.device.create_pipeline_layout(
            &wgpu::PipelineLayoutDescriptor {
                label: Some("pipeline_layout"),
                bind_group_layouts: &[],
                push_constant_ranges: &[],
            }
        );

        let shader: wgpu::ShaderModule = context.device.create_shader_module(wgpu::include_wgsl!("shader.wgsl"));

        let pipeline = context.device.create_render_pipeline(&wgpu::RenderPipelineDescriptor {
            label: Some("render_pipeline"),
            layout: Some(&pipeline_layout),
            vertex: wgpu::VertexState {
                module: &shader,
                entry_point: "vs_main",
                buffers: &[],
                compilation_options: Default::default(),
            },
            fragment: Some(wgpu::FragmentState {
                module: &shader,
                entry_point: "fs_main",
                targets: &[Some(wgpu::ColorTargetState {
                    format: surface.configuration.format,
                    blend: Some(wgpu::BlendState {
                        color: wgpu::BlendComponent::REPLACE,
                        alpha: wgpu::BlendComponent::REPLACE,
                    }),
                    write_mask: wgpu::ColorWrites::ALL,
                })],
                compilation_options: Default::default(),
            }),
            primitive: wgpu::PrimitiveState {
                topology: wgpu::PrimitiveTopology::TriangleList,
                strip_index_format: None,
                front_face: wgpu::FrontFace::Ccw,
                cull_mode: Some(wgpu::Face::Back),
                polygon_mode: wgpu::PolygonMode::Fill,
                unclipped_depth: false,
                conservative: false,
            },
            multisample: wgpu::MultisampleState {
                count: 1,
                mask: !0,
                alpha_to_coverage_enabled: false,
            },
            depth_stencil: None,
            multiview: None,
            cache: None,
        });

        Self {
            pipeline
        }
    }
    fn uptade(&mut self, app: &mut App, mut frame: Frame) {
        frame.clear(1.0, 0.0, 1.0, 1.0);

        {
            let mut render_pass = frame.encoder.begin_render_pass(&wgpu::RenderPassDescriptor {
                label: Some("render pass"),
                color_attachments: &[Some(
                    wgpu::RenderPassColorAttachment {
                        view: &frame.framebuffer_view,
                        resolve_target: None,
                        ops: wgpu::Operations {
                            load: wgpu::LoadOp::Load,
                            store: wgpu::StoreOp::Store
                        }
                    }    
                )],
                ..Default::default()
            });

            render_pass.set_pipeline(&self.pipeline);
            render_pass.draw(0..3, 0..1);
        }

        frame.present(app);
    }
}

fn main() {
    let app = App::new();
    app.run::<State>();
}
use wgpu::util::DeviceExt;

use crate::{ArcedRenderDevice, RenderSurface};

pub struct RenderableTexture {
    device: ArcedRenderDevice,
    texture: wgpu::Texture,
}

pub struct TextureSampler {
    sampler: wgpu::Sampler,
}

impl RenderableTexture {
    pub fn from_surface(device: &ArcedRenderDevice, surface: &RenderSurface) -> RenderableTexture {
        let texture_data = vec![
            255u8;
            (surface.configuration.width
                * surface.configuration.height
                * surface.format.target_pixel_byte_cost().unwrap())
                as usize
        ];
        let texture = device.device.create_texture_with_data(
            &device.queue,
            &wgpu::TextureDescriptor {
                view_formats: &[],
                label: Some("Texture 2D"),
                mip_level_count: 1,
                sample_count: 1,
                size: wgpu::Extent3d {
                    width: surface.configuration.width,
                    height: surface.configuration.height,
                    depth_or_array_layers: 1,
                },
                dimension: wgpu::TextureDimension::D2,
                format: surface.format,
                usage: wgpu::TextureUsages::TEXTURE_BINDING
                    | wgpu::TextureUsages::RENDER_ATTACHMENT,
            },
            wgpu::util::TextureDataOrder::LayerMajor,
            &texture_data,
        );

        RenderableTexture {
            device: device.clone(),
            texture,
        }
    }
    pub fn wgpu_texture(&self) -> &wgpu::Texture {
        &self.texture
    }
    pub fn clear_pass(&self, r: f64, g: f64, b: f64, a: f64) {
        let view = self.texture.create_view(&wgpu::TextureViewDescriptor {
            label: Some("Renderable Texture View"),
            ..Default::default()
        });

        let mut encoder =
            self.device
                .device
                .create_command_encoder(&wgpu::CommandEncoderDescriptor {
                    label: Some("Present Framebuffer Command Encoder"),
                });

        {
            _ = encoder.begin_render_pass(&wgpu::RenderPassDescriptor {
                label: Some("Present Framebuffer Render Pass"),
                color_attachments: &[Some(wgpu::RenderPassColorAttachment {
                    view: &view,
                    resolve_target: None,
                    ops: wgpu::Operations {
                        load: wgpu::LoadOp::Clear(wgpu::Color { r, g, b, a }),
                        store: wgpu::StoreOp::Store,
                    },
                })],
                depth_stencil_attachment: None,
                timestamp_writes: None,
                occlusion_query_set: None,
            });
        }

        self.device.queue.submit([encoder.finish()]);
    }
    pub fn clear_pass_with_encoder(
        &self,
        encoder: &mut wgpu::CommandEncoder,
        r: f64,
        g: f64,
        b: f64,
        a: f64,
    ) {
        let view = self.texture.create_view(&wgpu::TextureViewDescriptor {
            label: Some("Renderable Texture View"),
            ..Default::default()
        });

        {
            _ = encoder.begin_render_pass(&wgpu::RenderPassDescriptor {
                label: Some("Present Framebuffer Render Pass"),
                color_attachments: &[Some(wgpu::RenderPassColorAttachment {
                    view: &view,
                    resolve_target: None,
                    ops: wgpu::Operations {
                        load: wgpu::LoadOp::Clear(wgpu::Color { r, g, b, a }),
                        store: wgpu::StoreOp::Store,
                    },
                })],
                depth_stencil_attachment: None,
                timestamp_writes: None,
                occlusion_query_set: None,
            });
        }
    }
}

impl TextureSampler {
    pub fn new(device: &ArcedRenderDevice) -> TextureSampler {
        let sampler = device.device.create_sampler(&wgpu::SamplerDescriptor {
            label: Some("Sampler"),
            address_mode_u: wgpu::AddressMode::Repeat,
            address_mode_v: wgpu::AddressMode::Repeat,
            address_mode_w: wgpu::AddressMode::Repeat,
            mag_filter: wgpu::FilterMode::Linear,
            min_filter: wgpu::FilterMode::Nearest,
            ..Default::default()
        });
        TextureSampler {
            sampler,
        }
    }
    pub fn wgpu_sampler(&self) -> &wgpu::Sampler {
        &self.sampler
    }
}

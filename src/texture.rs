use wgpu::util::DeviceExt;

use crate::{ArcedRenderDevice, RenderSurface};

pub enum TextureFormat {
    R8Unorm,
    R8Snorm,
    R16Float,
    Rg8Unorm,
    Rg8Snorm,
    R32Float,
    Rg16Float,
    Rgba8Unorm,
    Rgba8UnormSrgb,
    Rgba8Snorm,
    Bgra8Unorm,
    Bgra8UnormSrgb,
    Rgb10a2Unorm,
    Rg11b10Float,
    Rg32Float,
    Rgba16Float,
    Rgba32Float,
    Stencil8,
    Depth16Unorm,
    Depth24Plus,
    Depth24PlusStencil8,
    Depth32Float,
}

pub struct Texture2D {
    device: ArcedRenderDevice,
    texture: wgpu::Texture,
    format: TextureFormat,
}

pub struct RenderableTexture {
    device: ArcedRenderDevice,
    texture: wgpu::Texture,
}

pub struct TextureSampler {
    device: ArcedRenderDevice,
    sampler: wgpu::Sampler,
}

impl TextureFormat {
    pub const fn to_wgpu(&self) -> wgpu::TextureFormat {
        match self {
            TextureFormat::R8Unorm => wgpu::TextureFormat::R8Unorm,
            TextureFormat::R8Snorm => wgpu::TextureFormat::R8Snorm,
            TextureFormat::R16Float => wgpu::TextureFormat::R16Float,
            TextureFormat::Rg8Unorm => wgpu::TextureFormat::Rg8Unorm,
            TextureFormat::Rg8Snorm => wgpu::TextureFormat::Rg8Snorm,
            TextureFormat::R32Float => wgpu::TextureFormat::R32Float,
            TextureFormat::Rg16Float => wgpu::TextureFormat::Rg16Float,
            TextureFormat::Rgba8Unorm => wgpu::TextureFormat::Rgba8Unorm,
            TextureFormat::Rgba8UnormSrgb => wgpu::TextureFormat::Rgba8UnormSrgb,
            TextureFormat::Rgba8Snorm => wgpu::TextureFormat::Rgba8Snorm,
            TextureFormat::Bgra8Unorm => wgpu::TextureFormat::Bgra8Unorm,
            TextureFormat::Bgra8UnormSrgb => wgpu::TextureFormat::Bgra8UnormSrgb,
            TextureFormat::Rgb10a2Unorm => wgpu::TextureFormat::Rgb10a2Unorm,
            TextureFormat::Rg11b10Float => wgpu::TextureFormat::Rg11b10Float,
            TextureFormat::Rg32Float => wgpu::TextureFormat::Rg32Float,
            TextureFormat::Rgba16Float => wgpu::TextureFormat::Rgba16Float,
            TextureFormat::Rgba32Float => wgpu::TextureFormat::Rgba32Float,
            TextureFormat::Stencil8 => wgpu::TextureFormat::Stencil8,
            TextureFormat::Depth16Unorm => wgpu::TextureFormat::Depth16Unorm,
            TextureFormat::Depth24Plus => wgpu::TextureFormat::Depth24Plus,
            TextureFormat::Depth24PlusStencil8 => wgpu::TextureFormat::Depth24PlusStencil8,
            TextureFormat::Depth32Float => wgpu::TextureFormat::Depth32Float,
        }
    }
    pub fn pixel_size(&self) -> u32 {
        self.to_wgpu().target_pixel_byte_cost().unwrap()
    }
}

impl Texture2D {
    pub fn empty(
        device: &ArcedRenderDevice,
        width: u32,
        height: u32,
        format: TextureFormat,
    ) -> Texture2D {
        let texture_data = vec![255u8; (width * height * format.pixel_size()) as usize];
        let texture = device.device.create_texture_with_data(
            &device.queue,
            &wgpu::TextureDescriptor {
                view_formats: &[],
                label: Some("Texture 2D"),
                mip_level_count: 1,
                sample_count: 1,
                size: wgpu::Extent3d {
                    width,
                    height,
                    depth_or_array_layers: 1,
                },
                dimension: wgpu::TextureDimension::D2,
                format: format.to_wgpu(),
                usage: wgpu::TextureUsages::TEXTURE_BINDING,
            },
            wgpu::util::TextureDataOrder::LayerMajor,
            &texture_data,
        );

        Texture2D {
            device: device.clone(),
            texture,
            format,
        }
    }
    pub fn from_data(
        device: &ArcedRenderDevice,
        data: &[u8],
        width: u32,
        height: u32,
        format: TextureFormat,
    ) -> Texture2D {
        let texture = device.device.create_texture_with_data(
            &device.queue,
            &wgpu::TextureDescriptor {
                view_formats: &[],
                label: Some("Texture 2D"),
                mip_level_count: 1,
                sample_count: 1,
                size: wgpu::Extent3d {
                    width,
                    height,
                    depth_or_array_layers: 1,
                },
                dimension: wgpu::TextureDimension::D2,
                format: format.to_wgpu(),
                usage: wgpu::TextureUsages::TEXTURE_BINDING,
            },
            wgpu::util::TextureDataOrder::LayerMajor,
            &data,
        );

        Texture2D {
            device: device.clone(),
            texture,
            format,
        }
    }
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
            device: device.clone(),
            sampler,
        }
    }
    pub fn wgpu_sampler(&self) -> &wgpu::Sampler {
        &self.sampler
    }
}

use wgpu::util::{DeviceExt, TextureDataOrder};

use crate::Context;

pub struct Sampler2D {
    sampler: wgpu::Sampler
}

pub struct Texture2D {
    texture: wgpu::Texture,
    format: wgpu::TextureFormat,
    view: wgpu::TextureView
}

impl Sampler2D {
    pub fn default(ctx: &Context) -> Sampler2D {
        let sampler = ctx.device.create_sampler(&wgpu::SamplerDescriptor
            {
                label: Some("Sampler 2D"),
                address_mode_u: wgpu::AddressMode::Repeat,
                address_mode_v: wgpu::AddressMode::Repeat,
                address_mode_w: wgpu::AddressMode::Repeat,
                mag_filter: wgpu::FilterMode::Linear,
                min_filter: wgpu::FilterMode::Linear,
                mipmap_filter: wgpu::FilterMode::Linear,
                ..Default::default()
            }
        );

        Sampler2D { sampler }
    }
    pub fn binding_type() -> wgpu::BindingType {
        wgpu::BindingType::Sampler(wgpu::SamplerBindingType::Filtering)
    }
    pub fn as_bind_resource(&self) -> wgpu::BindingResource<'_> {
        wgpu::BindingResource::Sampler(&self.sampler)
    }
}

impl Texture2D {
    pub fn new(
        ctx: &Context,
        data: &[u8],
        format: wgpu::TextureFormat,
        width: u32,
        height: u32
    ) -> Texture2D {
        
        let texture = ctx.device.create_texture_with_data(
        &ctx.queue, &wgpu::TextureDescriptor {
            label: None,
            size: wgpu::Extent3d {width, height, depth_or_array_layers: 1},
            mip_level_count: 1,
            sample_count: 1,
            dimension: wgpu::TextureDimension::D2,
            format,
            usage: wgpu::TextureUsages::TEXTURE_BINDING,
            view_formats: &[],
        }, TextureDataOrder::LayerMajor, data);

        let view = texture.create_view(&wgpu::TextureViewDescriptor::default());
        
        Texture2D {
            texture,
            format,
            view
        }
    }
    pub fn binding_type() -> wgpu::BindingType {
        wgpu::BindingType::Texture {
            multisampled: false,
            view_dimension: wgpu::TextureViewDimension::D2,
            sample_type: wgpu::TextureSampleType::Float { filterable: true },
        }
    }
    pub fn as_bind_resource(&self) -> wgpu::BindingResource<'_> {
        wgpu::BindingResource::TextureView(&self.view)
    }
}
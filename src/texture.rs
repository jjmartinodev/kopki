use crate::Context;

pub struct Sampler {
    sampler: wgpu::Sampler
}

pub struct Texture {
    texture: wgpu::Texture,
    format: wgpu::TextureFormat
}

impl Sampler {
    pub fn default(ctx: &Context) -> Sampler {
        let sampler = ctx.device.create_sampler(&wgpu::SamplerDescriptor
            {
                label: Some("Sampler"),
                address_mode_u: wgpu::AddressMode::Repeat,
                address_mode_v: wgpu::AddressMode::Repeat,
                address_mode_w: wgpu::AddressMode::Repeat,
                mag_filter: wgpu::FilterMode::Linear,
                min_filter: wgpu::FilterMode::Linear,
                mipmap_filter: wgpu::FilterMode::Linear,
                ..Default::default()
            }
        );

        Sampler { sampler }
    }
}

impl Texture {
    pub fn new(ctx: &Context) -> Texture {
        todo!()
    }
}
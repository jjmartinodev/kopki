use super::{context::Context, render::RenderResource};

pub struct GroupLayout {
    pub(crate)layout: wgpu::BindGroupLayout
}

pub struct Group {
    group: wgpu::BindGroup
}

impl GroupLayout {
    pub fn new(
        ctx: &Context,
        entries: &[wgpu::BindGroupLayoutEntry]
    ) -> GroupLayout {
        let layout = ctx.device.create_bind_group_layout(
            &wgpu::BindGroupLayoutDescriptor {
                label: Some("Bind Group Layout"),
                entries
            });
        GroupLayout { layout }
    }
}

impl Group {
    pub fn new(
        ctx: &Context,
        layout: &GroupLayout,
        resources: Vec<wgpu::BindingResource<'_>>
    ) -> Group {

        let mut entered = vec![];

        let mut binding = 0;
        for resource in resources {
            entered.push(wgpu::BindGroupEntry {
                binding,
                resource
            });
            binding += 1;
        }

        let group = ctx.device.create_bind_group(&wgpu::BindGroupDescriptor {
            label: Some("Bind Group"),
            layout: &layout.layout,
            entries: &entered
        });
        
        Group { group }
    }
    pub fn as_resource(&self) -> RenderResource<'_> {
        RenderResource::BindGroup { group: &self.group }
    }
}
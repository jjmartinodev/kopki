use crate::Context;

pub struct Group {
    group: wgpu::BindGroup,
    layout: wgpu::BindGroupLayout
}

impl Group {
    pub fn new(
        ctx: &Context,
        layout_entries: &[wgpu::BindGroupLayoutEntry],
        group_resources: &[wgpu::BindingResource]
    ) -> Group {
        let layout = ctx.device.create_bind_group_layout(
            &wgpu::BindGroupLayoutDescriptor {
                label: Some("Bind Group Layout"),
                entries: layout_entries
            });

        let group = ctx.device.create_bind_group(&wgpu::BindGroupDescriptor {
            label: Some("Bind Group"),
            layout: &layout,
            entries: &[

            ]
        });
        todo!()
    }
}
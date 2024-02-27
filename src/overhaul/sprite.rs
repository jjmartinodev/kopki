use bytemuck::{Pod, Zeroable};

use crate::core::{
    context::{Context, WindowSurface},
    mesh::StaticMesh,
    render::Pipeline,
    texture::TextureView2D
};

use super::transform::Transform;

#[repr(C)]
#[derive(Clone, Copy)]
struct SpriteVertex {
    position: [f32; 3],
    uv: [f32; 2],
}

unsafe impl Pod for SpriteVertex {}
unsafe impl Zeroable for SpriteVertex {}

const SPRITE_VERTICES: [SpriteVertex ; 4] = [
    SpriteVertex { position: [0.,0.,0.], uv: [0.,0.] },
    SpriteVertex { position: [1.,0.,0.], uv: [1.,0.] },
    SpriteVertex { position: [0.,1.,0.], uv: [0.,1.] },
    SpriteVertex { position: [1.,1.,0.], uv: [1.,1.] },
];

const SPRITE_INDICES: [u32; 6] = [0,1,2,3,1,2];

pub struct Sprite {
    quad_mesh: StaticMesh,
    view: TextureView2D
}

impl Sprite {
    pub fn new(ctx: &Context, view: TextureView2D, transform: Transform) -> Sprite {
        
        let quad_mesh = StaticMesh::new(ctx, &SPRITE_VERTICES, &SPRITE_INDICES);
        
        Sprite {
            quad_mesh,
            view
        }
    }
}
@vertex
fn vs_main(
    @builtin(vertex_index) in_vertex_index: u32,
) -> @builtin(position) vec4<f32> {
    let x = f32(1 - i32(in_vertex_index)) * 10.;
    let y = f32(i32(in_vertex_index & 1u) * 2 - 1) * 10.;
    return vec4<f32>(x, y, 0.0, 1.0);
}

@group(0) @binding(0)
var tris_texture: texture_2d<f32>;
@group(0) @binding(1)
var tris_sampler: sampler;

struct GlobalsUniform {
    framebuffer_size: vec2<f32>
};
@group(1) @binding(0) // 1.
var<uniform> globals: GlobalsUniform;

@fragment
fn fs_main(@builtin(position)position: vec4<f32>) -> @location(0) vec4<f32> {
    return textureSample(tris_texture, tris_sampler, position.xy / globals.framebuffer_size.xy);
}
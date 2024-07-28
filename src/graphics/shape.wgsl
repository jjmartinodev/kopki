struct Globals {
    framebuffer_size: vec2<f32>,
}

@group(0) @binding(0)
var<uniform> globals: Globals;

struct VertexInput {
    @location(0) position: vec2<f32>,
    @location(1) color: vec4<f32>,
};

struct VertexOutput {
    @builtin(position) clip_position: vec4<f32>,
    @location(0) color: vec4<f32>,
};

@vertex
fn vs_main(
    model: VertexInput,
) -> VertexOutput {
    var out: VertexOutput;
    out.color = model.color;
    var vertex_position: vec2<f32> = vec2<f32>(
        model.position.x / globals.framebuffer_size.x * 2. - 1.,
        model.position.y / globals.framebuffer_size.y * 2. - 1.
    );
    out.clip_position = vec4<f32>(vertex_position, 0.0, 1.0);
    return out;
}

@fragment
fn fs_main(in: VertexOutput) -> @location(0) vec4<f32> {
    return in.color;
}
#import bevy_sprite::mesh2d_view_bind_group
#import bevy_sprite::mesh2d_struct

struct Vertex {
    [[location(0)]] position: vec3<f32>;
    [[location(1)]] normal: vec3<f32>;
    [[location(2)]] uv: vec2<f32>;
    [[location(3)]] barycentric: vec3<f32>;
    [[location(4)]] color: vec3<f32>;
    [[location(5)]] dashed: vec2<f32>;
};

[[group(1), binding(0)]]
var<uniform> mesh: Mesh2d;
[[group(0), binding(0)]]
var<uniform> view: View;

struct VertexOutput {
    [[builtin(position)]] clip_position: vec4<f32>;
    [[location(0)]] position: vec3<f32>;
    [[location(1)]] uv: vec2<f32>;
    [[location(2)]] barycentric: vec3<f32>;
    [[location(3)]] color: vec3<f32>;
    [[location(4)]] dashed: vec2<f32>;
};

[[stage(vertex)]]
fn vertex(vertex: Vertex) -> VertexOutput {
    let world_position = mesh.model * vec4<f32>(vertex.position, 1.0);

    var out: VertexOutput;
    out.position = vertex.position;
    out.barycentric = vertex.barycentric;
    out.uv = vertex.uv;
    out.color = vertex.color;
    out.dashed = vertex.dashed;
    out.clip_position = view.view_proj * world_position;

    return out;
}

fn aastep(threshold: f32, distance: f32) -> f32 {
    var afwidth = fwidth(distance) * 0.5;
    return smoothStep(threshold - afwidth, threshold + afwidth, distance);
}

[[stage(fragment)]]
fn fragment(s
    in: VertexOutput
) -> [[location(0)]] vec4<f32> {
    var position_along = max(in.barycentric.x, in.barycentric.y);
    if (in.barycentric.y < in.barycentric.x && in.barycentric.y < in.barycentric.z) {
        position_along = 1.0 - position_along;
    }
    var dash_repeats = in.dashed.x;
    var dash_length = in.dashed.y;
    var offset = 1.0 / dash_repeats * dash_length / 2.0;
    var offset = offset + (1.0 / dash_repeats / 2.0);
    var pattern = fract((position_along + offset) * dash_repeats);
    var edge = 1.0 - aastep(dash_length, pattern);
    return vec4<f32>(1.0, 1.0, 1.0, edge);
}
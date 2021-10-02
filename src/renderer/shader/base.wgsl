// Vertex shader

[[block]]
struct ModelUniform {
    model_mat: mat4x4<f32>;
};
[[group(0), binding(0)]]
var<uniform> model: ModelUniform;

[[block]]
struct CameraUniform {
    view_proj_mat: mat4x4<f32>;
};
[[group(1), binding(0)]]
var<uniform> camera: CameraUniform;

struct VertexInput {
    [[location(0)]] position: vec3<f32>;
};

struct VertexOutput {
    [[location(0)]] position: vec3<f32>;
    [[builtin(position)]] clip_position: vec4<f32>;
};


[[stage(vertex)]]
fn main(
    in: VertexInput,
) -> VertexOutput {
    var out: VertexOutput;
    out.position = in.position;
    out.clip_position = camera.view_proj_mat * model.model_mat * vec4<f32>(in.position, 1.0);
    return out;
}

// Fragment shader

[[stage(fragment)]]
fn main(in: VertexOutput) -> [[location(0)]] vec4<f32> {
    var color = (in.position + vec3<f32>(1.0, 1.0, 1.0)) * 0.5;
    return vec4<f32>(color.x, color.y, 0.0, 1.0);
}

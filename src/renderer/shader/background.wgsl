// Vertex shader

[[block]]
struct Globals {
    view_proj_mat: mat4x4<f32>;
    sprite_idx: u32;
    sprite_size: array<u32, 2>;
    sprite_sheet_size: array<u32, 2>;
    player_health: f32;
};
[[group(0), binding(2)]]
var<uniform> globals: Globals;

struct VertexInput {
    [[location(0)]] position: vec3<f32>;
    [[location(1)]] tex_coords: vec2<f32>;
};

struct VertexOutput {
    [[builtin(position)]] clip_position: vec4<f32>;
    [[location(0)]] tex_coords: vec2<f32>;
    [[location(1)]] sprite_tex_coords: vec2<f32>;
    [[location(2)]] player_health: f32;
};

struct SpriteCell {
    x: u32;
    y: u32;
};

struct SpriteRect {
    x_min: f32;
    x_max: f32;
    y_min: f32;
    y_max: f32;
};

fn normalize_val(val: u32, min: u32, max: u32) -> f32 {
    return f32(val - min) / f32(max - min);
}

fn get_sprite_cell(
    sprite_idx: u32,
    sprite_size: array<u32, 2>,
    sprite_sheet_size: array<u32, 2>,
) -> SpriteCell {
    var cell: SpriteCell;
    var width = (sprite_sheet_size[0] / sprite_size[0]);
    cell.x = sprite_idx % width;
    cell.y = sprite_idx / width;
    return cell;
}

fn get_sprite_cell_rect(
    sprite_cell: SpriteCell,
    sprite_size: array<u32, 2>,
    sprite_sheet_size: array<u32, 2>,
) -> SpriteRect {
    var x_min = sprite_size[0] * sprite_cell.x;
    var x_max = x_min + sprite_size[0];
    var y_min = sprite_size[1]  * sprite_cell.y;
    var y_max = y_min + sprite_size[1];

    var rect: SpriteRect;
    rect.x_min = normalize_val(x_min, 0u, sprite_sheet_size[0]);
    rect.x_max = normalize_val(x_max, 0u, sprite_sheet_size[0]);
    rect.y_min = normalize_val(y_min, 0u, sprite_sheet_size[1]);
    rect.y_max = normalize_val(y_max, 0u, sprite_sheet_size[1]);
    return rect;
}

fn to_sprite_coords(
    tex_coords: vec2<f32>,
    sprite_idx: u32,
    sprite_size: array<u32, 2>,
    sprite_sheet_size: array<u32, 2>,
) -> vec2<f32> {
    var cell = get_sprite_cell(sprite_idx, sprite_size, sprite_sheet_size);
    var rect = get_sprite_cell_rect(cell, sprite_size, sprite_sheet_size);

    var coord_x = mix(rect.x_min, rect.x_max, tex_coords.x);
    var coord_y = mix(rect.y_min, rect.y_max, tex_coords.y);

    return vec2<f32>(coord_x, coord_y);
}

[[stage(vertex)]]
fn main(
    in: VertexInput,
) -> VertexOutput {
    var out: VertexOutput;
    out.tex_coords = in.tex_coords;
    out.sprite_tex_coords = to_sprite_coords(
        in.tex_coords,
        globals.sprite_idx,
        globals.sprite_size,
        globals.sprite_sheet_size,
    );
    out.clip_position = vec4<f32>(in.position, 1.0);
    out.player_health = globals.player_health;
    return out;
}

// Fragment shader

let HEALTH_BAR_SIZE: f32 = 0.05;

[[group(0), binding(0)]]
var t_diffuse: texture_2d<f32>;
[[group(0), binding(1)]]
var s_diffuse: sampler;

fn health_bar_at_coord(tex_coords: vec2<f32>, player_health: f32) -> f32 {
    if (tex_coords.y < HEALTH_BAR_SIZE && tex_coords.x < player_health) {
        return 0.0;
    } else {
        return 1.0;
    }
}

[[stage(fragment)]]
fn main(in: VertexOutput) -> [[location(0)]] vec4<f32> {
    var texture_color = textureSample(t_diffuse, s_diffuse, in.sprite_tex_coords);
    var health_bar_color = vec4<f32>(1.0, 1.0, 1.0, 1.0);

    var health_bar = health_bar_at_coord(in.tex_coords, in.player_health);

    return mix(health_bar_color, texture_color, health_bar);
}

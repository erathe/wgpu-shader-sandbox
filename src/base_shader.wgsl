struct VertexOutput {
    @location(0) uv: vec2<f32>,
    @builtin(position) clip_position: vec4<f32>,
};



@vertex
fn vs_main(
	@builtin(vertex_index) vi: u32
) -> VertexOutput {
    var out: VertexOutput;
    // Generate a triangle that covers the whole screen
    out.uv = vec2<f32>(
        f32((vi << 1u) & 2u),
        f32(vi & 2u),
    );
    out.clip_position = vec4<f32>(out.uv * 2.0 - 1.0, 0.0, 1.0);
    // We need to invert the y coordinate so the image
    // is not upside down
    // out.uv.y = 1.0 - out.uv.y;
    out.uv = (out.uv - 0.5) * 2.0;
    // out.uv.x *= 1600.0/1200.0;
    return out;
}

struct System {
    screen: vec2<f32>,
    mouse: vec2<f32>,
    time: f32,
}
@group(0) @binding(0)
var<uniform> system: System;

@fragment
fn fs_main(vs: VertexOutput) -> @location(0) vec4<f32> {

    var col: vec3<f32> = vec3(1.0, 0.0, 0.0);

    var uv = vs.uv;
    // fix aspect ratio
    uv.x *= system.screen.x / system.screen.y;

    var d = length(uv) - 0.5 + (system.mouse.x / system.screen.x);
    d = sin(d * 8. + system.time) / 8.;
    d = abs(d);

    // d = smoothstep(0.0, 0.1, d);
    d = 0.02 / d;

    col *= d;
    
    return vec4(col, 1.0);
}


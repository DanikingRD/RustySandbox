
struct VertexIn {
    @location(0) pos: vec3<f32>,
    @location(1) color: vec3<f32>
}

struct VertexOut {
    @builtin(position) pos: vec4<f32>,
    @location(0) color: vec3<f32>
}

@vertex 
fn vs_main(input: VertexIn) -> VertexOut {
    var v_out: VertexOut;
    v_out.pos = vec4<f32>(input.pos, 1.0);
    v_out.color = input.color;
    return v_out;
}

@fragment
fn fs_main(input: VertexOut) ->  @location(0) vec4<f32> {
    return vec4<f32>(input.color, 1.0);
}
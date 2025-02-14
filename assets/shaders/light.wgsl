// @group(2) @binding(0) var<uniform> pos: vec3<f32>

@fragment
fn fragment() ->  @location(0) vec4<f32> {
    return vec4(1.0, 0.87, 0.13, 1.0);
}
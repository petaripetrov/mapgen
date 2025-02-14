// this is made available to the importing module
// const COLOR_MULTIPLIER: vec4<f32> = vec4<f32>(1.0, 1.0, 1.0, 0.5);

fn diffuse(
    pos: vec3<f32>,
    normal: vec3<f32>,
    l_col: vec3<f32>,
    l_pos: vec3<f32>,
    l_int: f32,
) -> vec3<f32> {
    let norm = normalize(normal);
    let light_dir = normalize(l_pos - pos);
    let diffuse = max(dot(norm, light_dir), 0.0) * l_int;

    return l_col * diffuse;
}

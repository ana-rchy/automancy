struct Uniform {
    light_pos: vec4<f32>,
    light_color: vec4<f32>,
    world: mat4x4<f32>,
}

@group(0) @binding(0)
var<uniform> ubo: Uniform;

struct VertexInput {
    @location(0) pos: vec3<f32>,
    @location(1) color: vec4<f32>,
    @location(2) normal: vec3<f32>,
}

struct InstanceInput {
    @location(3) color_offset: vec4<f32>,
    @location(4) model_matrix_0: vec4<f32>,
    @location(5) model_matrix_1: vec4<f32>,
    @location(6) model_matrix_2: vec4<f32>,
    @location(7) model_matrix_3: vec4<f32>,
}

struct VertexOutput {
    @builtin(position) pos: vec4<f32>,
    @location(0) color: vec4<f32>,
    @location(1) model_pos: vec3<f32>,
    @location(2) normal: vec3<f32>,
}

@vertex
fn vs_main(
    model: VertexInput,
    instance: InstanceInput,
) -> VertexOutput {
    let model_matrix = mat4x4<f32>(
       instance.model_matrix_0,
       instance.model_matrix_1,
       instance.model_matrix_2,
       instance.model_matrix_3,
   );

    var out: VertexOutput;

    let model_pos = model_matrix * vec4(model.pos, 1.0);
    out.pos = ubo.world * model_pos;
    out.model_pos = model_pos.xyz / model_pos.w;
    out.normal = model.normal;

    out.color = ((1.0 - instance.color_offset.a) * model.color) +
                      ((instance.color_offset.a) * instance.color_offset);

    return out;
}

@fragment
fn fs_main(in: VertexOutput) -> @location(0) vec4<f32> {
    let light_color = ubo.light_color * 0.15;
    let light_dir = ubo.light_pos.xyz - in.model_pos;
    let light_distance = length(light_dir);

    let norm = normalize(in.normal);
    let reflected = -reflect(normalize(light_dir), norm);
    let eye = normalize(-in.model_pos);
    let halfway = normalize(light_dir + eye);

    var diffuse_intensity = max(dot(norm, reflected), 0.0);
    diffuse_intensity = pow(diffuse_intensity, 4.0);
    let diffuse = light_color * diffuse_intensity;

    let specular_intensity = dot(norm, halfway);
    let specular = light_color * specular_intensity;

    let color = vec4(0.5) + diffuse + specular;

    return color * in.color;
}
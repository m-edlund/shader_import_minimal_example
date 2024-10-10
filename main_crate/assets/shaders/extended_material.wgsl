#import bevy_pbr::{
    pbr_fragment::pbr_input_from_standard_material,
    forward_io::{VertexOutput, FragmentOutput},
}

// #ifdef OPTIONAL_SHADER
// #import optional_shader_function::optional_function;
// #endif

@group(2) @binding(100) var<uniform> extended_color: vec4<f32>;

@fragment
fn fragment(
    in: VertexOutput,
    @builtin(front_facing) is_front: bool,
) -> FragmentOutput {
    var out: FragmentOutput;

    // generate a PbrInput struct from the StandardMaterial bindings
    var pbr_input = pbr_input_from_standard_material(in, is_front);

    out.color = extended_color;

// #ifdef OPTIONAL_SHADER
//     out.color = optional_function();
// #endif

    return out;
}

#import bevy_pbr::forward_io::VertexOutput
// we can import items from shader modules in the assets folder with a quoted path
#import "shaders/custom_material_import.wgsl"::COLOR_MULTIPLIER

@group(2) @binding(0) var<uniform> material_color: vec4<f32>;
@group(2) @binding(1) var material_color_texture: texture_2d<f32>;
@group(2) @binding(2) var material_color_sampler: sampler;
@group(2) @binding(5) var<uniform> mesh_dimensions: vec2<f32>;

@fragment
fn fragment(
    mesh: VertexOutput,
) -> @location(0) vec4<f32> {
    let uv = mesh.uv * mesh_dimensions;
    return material_color * textureSample(material_color_texture, material_color_sampler, uv) * COLOR_MULTIPLIER;
}
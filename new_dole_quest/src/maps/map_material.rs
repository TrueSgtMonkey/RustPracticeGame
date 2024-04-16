use bevy::prelude::*;
use bevy::render::render_resource::{AsBindGroup, ShaderRef};
use bevy::sprite::{Material2d, MaterialMesh2dBundle};

#[derive(AsBindGroup, Debug, Clone, Asset, TypePath)]
pub struct MapMaterial {
    // Uniform bindings must implement `ShaderType`, which will be used to convert the value to
    // its shader-compatible equivalent. Most core math types already implement `ShaderType`.
    #[uniform(0)]
    pub color: Color,
    // Images can be bound as textures in shaders. If the Image's sampler is also needed, just
    // add the sampler attribute with a different binding index.
    #[texture(1)]
    #[sampler(2)]
    pub color_texture: Handle<Image>,
}

// All functions on `Material2d` have default impls. You only need to implement the
// functions that are relevant for your material.
impl Material2d for MapMaterial {
    fn fragment_shader() -> ShaderRef {
        "
            struct CustomMaterial {
                color: vec4<f32>,
            }

            @group(2) @binding(0) var<uniform> material: CustomMaterial;
            @group(2) @binding(1) var color_texture: texture_2d<f32>;
            @group(2) @binding(2) var color_sampler: sampler;
        ".into()
    }
}

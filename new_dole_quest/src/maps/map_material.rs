use std::vec;

use bevy::prelude::*;
use bevy::render::render_resource::{AsBindGroup, ShaderRef};
use bevy::sprite::Material2d;

#[derive(Resource, AsBindGroup, Debug, Clone, Asset, TypePath)]
pub struct MapMaterial {
    // Uniform bindings must implement `ShaderType`, which will be used to convert the value to
    // its shader-compatible equivalent. Most core math types already implement `ShaderType`.
    #[uniform(0)]
    pub color: Color,
    // Images can be bound as textures in shaders. If the Image's sampler is also needed, just
    // add the sampler attribute with a different binding index.
    #[texture(1)]
    #[sampler(2)]
    pub texture: Handle<Image>,

    #[uniform(5)]
    pub mesh_dimensions: Vec2,
}

// All functions on `Material2d` have default impls. You only need to implement the
// functions that are relevant for your material.
impl Material2d for MapMaterial {
    fn fragment_shader() -> ShaderRef {
        "shaders/map_material.wgsl".into()
    }
}

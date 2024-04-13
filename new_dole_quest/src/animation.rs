use bevy::prelude::*; // need this even in submodules

#[derive(Component)]
pub struct AnimationIndices {
    pub first: usize,
    pub last: usize,
}
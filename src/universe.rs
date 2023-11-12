use bevy::prelude::Resource;

// const GRAVITATIONAL_CONSTANT: f32 = 6.674e-11;
const GRAVITATIONAL_CONSTANT: f32 = 0.01;

#[derive(Resource, Debug)]
pub struct Universe {
    pub gravity_constant: f32,
}

impl Default for Universe {
    fn default() -> Self {
        Self {
            gravity_constant: GRAVITATIONAL_CONSTANT,
        }
    }
}
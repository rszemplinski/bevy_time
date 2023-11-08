use bevy::prelude::Resource;

const GRAVITY_CONSTANT: f32 = 0.0001;

#[derive(Resource, Debug)]
pub struct Universe {
    pub gravity_constant: f32,
}

impl Default for Universe {
    fn default() -> Self {
        Self {
            gravity_constant: GRAVITY_CONSTANT,
        }
    }
}
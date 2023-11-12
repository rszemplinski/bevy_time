use bevy::prelude::{Component, Reflect, Vec3};
use ringbuffer::ConstGenericRingBuffer;

#[derive(Component, Copy, Clone, Reflect, Default)]
pub struct CelestialBody {
    pub mass: f32,
    pub radius: f32,
    pub velocity: Vec3,
    pub position: Vec3,
}

#[derive(Component, Clone, Default, Debug)]
pub struct Trail(pub ConstGenericRingBuffer<Vec3, 1024>);

impl CelestialBody {
    pub fn update_position(&mut self, dt: f32) {
        self.position += self.velocity * dt;
    }
}
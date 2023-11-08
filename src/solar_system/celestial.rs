use bevy::prelude::{Bundle, Component, PbrBundle};
use bevy_xpbd_3d::prelude::{LinearVelocity, Mass};

#[derive(Component, Default, Debug)]
pub struct Radius(pub f32);

#[derive(Component, Default, Debug)]
pub struct Name(pub String);

#[derive(Bundle, Default)]
pub struct CelestialBody {
    pub name: Name,
    pub mass: Mass,
    pub radius: Radius,
    pub velocity: LinearVelocity,
    pub pbr: PbrBundle,
}
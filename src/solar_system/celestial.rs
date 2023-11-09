use bevy::prelude::{Bundle, Component, Name, PbrBundle, Reflect};
use bevy_xpbd_3d::prelude::{LinearVelocity, Mass};

#[derive(Component, Reflect, Default, Debug)]
pub struct Radius(pub f32);

#[derive(Component, Default, Debug)]
pub struct CelestialBody;

#[derive(Bundle, Default)]
pub struct CelestialBodyBundle {
    pub name: Name,
    pub mass: Mass,
    pub radius: Radius,
    pub velocity: LinearVelocity,
    pub pbr: PbrBundle,
}
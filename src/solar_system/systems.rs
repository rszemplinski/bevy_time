use bevy::prelude::*;
use ringbuffer::RingBuffer;
use crate::solar_system::celestial::*;
use crate::universe::Universe;

pub fn check_for_changes(
    mut query: Query<(&CelestialBody, &mut Transform), Changed<CelestialBody>>,
) {
    for (celestial, mut transform) in query.iter_mut() {
        if celestial.radius != transform.scale.x {
            transform.scale = Vec3::splat(celestial.radius);
        }

        if celestial.position != transform.translation {
            transform.translation = celestial.position;
        }
    }
}
fn calculate_gravitational_force(body1: &CelestialBody, body2: &CelestialBody, gravity_constant: f32) -> Vec3 {
    let distance_vec = body2.position - body1.position;
    let distance = distance_vec.length();
    let force_magnitude = gravity_constant * (body1.mass * body2.mass) / distance.powi(2);
    distance_vec.normalize() * force_magnitude
}

pub fn update_orbit(mut query: Query<(Entity, &mut CelestialBody)>, universe: Res<Universe>, time: Res<Time>) {
    let dt = time.delta_seconds();
    let mut iter = query.iter_combinations_mut();

    while let Some([(_, mut body_a), (_, mut body_b)]) = iter.fetch_next() {
        let force = calculate_gravitational_force(&body_a, &body_b, universe.gravity_constant);

        let body_a_mass = body_a.mass;
        let body_b_mass = body_b.mass;

        body_a.velocity += force / body_a_mass * dt;
        body_b.velocity -= force / body_b_mass * dt;
    }

    // Update the positions of the bodies
    for (_, mut body) in query.iter_mut() {
        body.update_position(dt);
    }
}

const MINIMUM_ANGLE: f32 = 1.019_726_74; // == acos(5 degrees)

pub fn update_trails(
) {

}
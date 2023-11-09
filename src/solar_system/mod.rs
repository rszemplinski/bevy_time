mod celestial;
mod debug;

use bevy::input::common_conditions::input_toggle_active;
use bevy::prelude::*;
use bevy_xpbd_3d::prelude::{AsyncCollider, ColliderDensity, ComputedCollider, Gravity, LinearVelocity, Mass, RigidBody};
use crate::solar_system::celestial::{CelestialBody, CelestialBodyBundle, Radius};
use crate::solar_system::debug::CelestialBodyDebugPlugin;

fn setup(
    mut commands: Commands,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<StandardMaterial>>,
) {
    // // Plane
    // commands.spawn((
    //     RigidBody::Static,
    //     AsyncCollider(ComputedCollider::ConvexHull),
    //     PbrBundle {
    //         mesh: meshes.add(shape::Circle::new(4.0).into()),
    //         material: materials.add(Color::WHITE.into()),
    //         transform: Transform::from_rotation(Quat::from_rotation_x(-std::f32::consts::FRAC_PI_2)),
    //         ..default()
    //     },
    // ));
    // // Cube
    // commands.spawn((
    //     RigidBody::Dynamic,
    //     AngularVelocity(Vec3::new(2.5, 3.4, 1.6)),
    //     Collider::cuboid(1.0, 1.0, 1.0),
    //     PbrBundle {
    //         mesh: meshes.add(Mesh::from(shape::Cube { size: 1.0 })),
    //         material: materials.add(Color::rgb(0.8, 0.7, 0.6).into()),
    //         transform: Transform::from_xyz(0.0, 4.0, 0.0),
    //         ..default()
    //     },
    // ));

    const SUN_RADIUS: f32 = 1.0;
    commands.spawn((
        CelestialBodyBundle {
            name: "Sun".into(),
            mass: Mass(500.),
            radius: Radius(SUN_RADIUS),
            velocity: LinearVelocity(Vec3::ZERO),
            pbr: PbrBundle {
                mesh: meshes.add(Mesh::try_from(shape::Icosphere {
                    radius: SUN_RADIUS,
                    subdivisions: 5,
                }).unwrap()),
                material: materials.add(StandardMaterial {
                    base_color: Color::ORANGE_RED,
                    emissive: (Color::ORANGE_RED * 2.),
                    ..default()
                }),
                transform: Transform::from_scale(Vec3::splat(1.0)),
                ..default()
            },
        },
        AsyncCollider(ComputedCollider::ConvexHull),
        RigidBody::Dynamic,
        ColliderDensity(0.0),
        CelestialBody
    )).with_children(|p| {
        p.spawn(PointLightBundle {
            point_light: PointLight {
                color: Color::WHITE,
                intensity: 400.0,
                range: 100.0,
                radius: 1.,
                ..default()
            },
            ..default()
        });
    });

    // light
    // commands.spawn(PointLightBundle {
    //     point_light: PointLight {
    //         intensity: 1500.0,
    //         shadows_enabled: true,
    //         ..default()
    //     },
    //     transform: Transform::from_xyz(4.0, 8.0, 4.0),
    //     ..default()
    // });
}

fn check_for_changes(
    mut query: Query<(&Radius, &mut Transform), Changed<Radius>>,
) {
    for (radius, mut transform) in query.iter_mut() {
        transform.scale = Vec3::splat(radius.0);
    }
}

pub struct SolarSystemPlugin;

impl Plugin for SolarSystemPlugin {
    fn build(&self, app: &mut App) {
        app
            .register_type::<Radius>()
            .insert_resource(Gravity(Vec3::splat(0.)))
            .add_plugins(
                CelestialBodyDebugPlugin::default()
                    .run_if(input_toggle_active(true, KeyCode::F4)))
            .add_systems(Startup, setup)
            .add_systems(FixedUpdate, check_for_changes);
    }
}
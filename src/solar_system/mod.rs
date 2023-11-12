mod celestial;
mod debug;
mod systems;

use bevy::input::common_conditions::input_toggle_active;
use bevy::prelude::*;
use ringbuffer::ConstGenericRingBuffer;
use crate::solar_system::celestial::{CelestialBody, Trail};
use crate::solar_system::debug::CelestialBodyDebugPlugin;
use crate::solar_system::systems::{check_for_changes, update_orbit, update_trails};

fn setup(
    mut commands: Commands,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<StandardMaterial>>,
) {
    const SUN_RADIUS: f32 = 2.5;
    commands.spawn((
        Name::new("Sun"),
        PbrBundle {
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
        CelestialBody {
            mass: 1_000_000.,
            radius: SUN_RADIUS,
            velocity: Vec3::ZERO,
            position: Vec3::ZERO,
        },
    )).with_children(|p| {
        p.spawn(PointLightBundle {
            point_light: PointLight {
                color: Color::WHITE,
                intensity: 5_000.0,
                range: 10_000.0,
                radius: SUN_RADIUS,
                shadows_enabled: true,
                ..default()
            },
            ..default()
        });
    });

    commands.spawn((
        Name::new("Body 1"),
        CelestialBody {
            mass: 100.,
            radius: 1.,
            velocity: Vec3::new(-5., 0., 5.),
            position: Vec3::new(150., 0., 0.),
        },
        PbrBundle {
            mesh: meshes.add(Mesh::try_from(shape::Icosphere {
                radius: 1.,
                subdivisions: 5,
            }).unwrap()),
            material: materials.add(StandardMaterial {
                base_color: Color::BLUE,
                ..default()
            }),
            ..default()
        },
        Trail(ConstGenericRingBuffer::<Vec3, 1024>::new()),
    ));

    // commands.spawn((
    //     Name::new("Body 2"),
    //     PbrBundle {
    //         mesh: meshes.add(Mesh::try_from(shape::Icosphere {
    //             radius: 1.,
    //             subdivisions: 5,
    //         }).unwrap()),
    //         material: materials.add(StandardMaterial {
    //             base_color: Color::GREEN,
    //             ..default()
    //         }),
    //         ..default()
    //     },
    //     CelestialBody {
    //         mass: 100.,
    //         radius: 1.,
    //         velocity: Vec3::new(-10., 0., 10.),
    //         position: Vec3::new(0., 0., -30.),
    //     },
    //     Trail(ConstGenericRingBuffer::<Vec3, 1024>::new()),
    // ))
    //     .with_children(|p| {
    //         p.spawn((
    //             PolylineBundle {
    //                 polyline: polylines.add(Polyline {
    //                     vertices: Vec::with_capacity(1024),
    //                 }),
    //                 material: polyline_materials.add(PolylineMaterial {
    //                     width: (100f32 * 0.1).powf(1.8),
    //                     color: Color::GREEN,
    //                     perspective: true,
    //                     ..Default::default()
    //                 }),
    //                 ..Default::default()
    //             },
    //         ));
    //     });
}

pub struct SolarSystemPlugin;

impl Plugin for SolarSystemPlugin {
    fn build(&self, app: &mut App) {
        app
            .register_type::<CelestialBody>()
            .add_plugins(
                CelestialBodyDebugPlugin::default()
                    .run_if(input_toggle_active(true, KeyCode::F4)))
            .add_systems(Startup, setup)
            .add_systems(FixedUpdate, (
                check_for_changes,
                update_orbit,
                update_trails
            ));
    }
}
mod celestial;

use bevy::prelude::*;
use bevy_egui::{EguiSet};
use bevy_inspector_egui::bevy_egui::EguiContexts;
use bevy_xpbd_3d::prelude::{AngularVelocity, AsyncCollider, Collider, ComputedCollider, RigidBody};

fn setup(
    mut commands: Commands,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<StandardMaterial>>,
) {
    // Plane
    commands.spawn((
        RigidBody::Static,
        AsyncCollider(ComputedCollider::ConvexHull),
        PbrBundle {
            mesh: meshes.add(shape::Circle::new(4.0).into()),
            material: materials.add(Color::WHITE.into()),
            transform: Transform::from_rotation(Quat::from_rotation_x(-std::f32::consts::FRAC_PI_2)),
            ..default()
        },
    ));
    // Cube
    commands.spawn((
        RigidBody::Dynamic,
        AngularVelocity(Vec3::new(2.5, 3.4, 1.6)),
        Collider::cuboid(1.0, 1.0, 1.0),
        PbrBundle {
            mesh: meshes.add(Mesh::from(shape::Cube { size: 1.0 })),
            material: materials.add(Color::rgb(0.8, 0.7, 0.6).into()),
            transform: Transform::from_xyz(0.0, 4.0, 0.0),
            ..default()
        },
    ));
    // light
    commands.spawn(PointLightBundle {
        point_light: PointLight {
            intensity: 1500.0,
            shadows_enabled: true,
            ..default()
        },
        transform: Transform::from_xyz(4.0, 8.0, 4.0),
        ..default()
    });
}

fn mouse_click_system(
    mouse_input: Res<Input<MouseButton>>,
    contexts: EguiContexts,
) {
    if contexts.ctx().wants_pointer_input() {
        return;
    }

    if mouse_input.pressed(MouseButton::Left) {
    }
}

pub struct ChasePlugin;

impl Plugin for ChasePlugin {
    fn build(&self, app: &mut App) {
        app
            .add_systems(Startup, setup)
            .add_systems(Update, (mouse_click_system)
                .chain().after(EguiSet::ProcessInput));
    }
}
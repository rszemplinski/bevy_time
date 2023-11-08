use bevy::input::mouse::MouseButtonInput;
use bevy::prelude::*;
use bevy::sprite::MaterialMesh2dBundle;
use bevy_cursor::CursorInfo;
use bevy_egui::{EguiContext, EguiSet};
use bevy_inspector_egui::bevy_egui::EguiContexts;

#[derive(Resource, Default, Debug, Clone)]
pub struct Target {
    position: Vec3,
}

#[derive(Component)]
pub struct Mover;

fn setup(
    mut commands: Commands,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<StandardMaterial>>,
) {
    // circular base
    commands.spawn(PbrBundle {
        mesh: meshes.add(shape::Circle::new(4.0).into()),
        material: materials.add(Color::WHITE.into()),
        transform: Transform::from_rotation(Quat::from_rotation_x(-std::f32::consts::FRAC_PI_2)),
        ..default()
    });
    // cube
    commands.spawn(PbrBundle {
        mesh: meshes.add(Mesh::from(shape::Cube { size: 1.0 })),
        material: materials.add(Color::rgb_u8(124, 144, 255).into()),
        transform: Transform::from_xyz(0.0, 0.5, 0.0),
        ..default()
    });
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
    cursor: Res<CursorInfo>,
    mut target: ResMut<Target>,
    mut contexts: EguiContexts,
) {
    if contexts.ctx().wants_pointer_input() {
        return;
    }

    if mouse_input.pressed(MouseButton::Left) {
        if let Some(position) = cursor.position() {
            target.position = Vec3::new(position.x, position.y, 0.);
        }
    }
}

fn draw_target(
    mut gizmos: Gizmos,
    target: Res<Target>,
) {
    gizmos
        .circle(Vec3::ZERO, Vec3::Y, 3.1, Color::NAVY)
        .segments(64);
}

fn move_towards_target_system(
    time: Res<Time>,
    target: Res<Target>,
    mut query_mover: Query<&mut Transform, With<Mover>>,
) {
    for mut transform in query_mover.iter_mut() {
        let direction = target.position - transform.translation;
        if direction.length() > 1.0 {
            transform.translation += direction.normalize() * time.delta_seconds() * 100.0;
        }
    }
}

pub struct ChasePlugin;

impl Plugin for ChasePlugin {
    fn build(&self, app: &mut App) {
        app
            .init_resource::<Target>()
            .add_systems(Startup, setup)
            .add_systems(Update, (mouse_click_system, move_towards_target_system, draw_target)
                .chain().after(EguiSet::ProcessInput));
    }
}
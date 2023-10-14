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
    mut materials: ResMut<Assets<ColorMaterial>>,
) {
    for i in 0..1 {
        commands.spawn(MaterialMesh2dBundle {
            mesh: meshes.add(shape::Circle::new(50.).into()).into(),
            material: materials.add(ColorMaterial::from(Color::PURPLE)),
            transform: Transform::from_translation(Vec3::new(0., 0., 0.)),
            ..default()
        }).insert(Mover);
    }
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
    // Draw X on target position
    gizmos.line(
        target.position + Vec3::new(-10., -10., 0.),
        target.position + Vec3::new(10., 10., 0.),
        Color::RED,
    );
    gizmos.line(
        target.position + Vec3::new(-10., 10., 0.),
        target.position + Vec3::new(10., -10., 0.),
        Color::RED,
    );
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
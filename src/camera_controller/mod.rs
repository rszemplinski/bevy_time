use bevy::core_pipeline::bloom::BloomSettings;
use bevy::core_pipeline::tonemapping::Tonemapping;
use bevy::prelude::*;
use bevy_panorbit_camera::{PanOrbitCamera, PanOrbitCameraPlugin};

pub struct CameraControllerPlugin;

#[derive(Component)]
pub struct MainCamera;

fn setup_camera(
    mut commands: Commands,
) {
    commands.spawn((
        Camera3dBundle {
            camera: Camera {
                hdr: true,
                ..default()
            },
            tonemapping: Tonemapping::BlenderFilmic,
            transform: Transform::from_xyz(-2.5, 4.5, 9.0).looking_at(Vec3::ZERO, Vec3::Y),
            ..default()
        },
        BloomSettings::default(),
        PanOrbitCamera::default(),
        MainCamera,
    ));
}

impl Plugin for CameraControllerPlugin {
    fn build(&self, app: &mut App) {
        app
            .add_plugins(PanOrbitCameraPlugin)
            .add_systems(Startup, setup_camera);
    }
}
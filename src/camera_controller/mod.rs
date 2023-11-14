use bevy::core_pipeline::bloom::BloomSettings;
use bevy::core_pipeline::tonemapping::Tonemapping;
use bevy::prelude::*;
use smooth_bevy_cameras::controllers::orbit::{OrbitCameraBundle, OrbitCameraController, OrbitCameraPlugin};
use smooth_bevy_cameras::LookTransformPlugin;

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
            transform: Transform::from_xyz(-2.5, 4.5, 9.0)
                .looking_at(Vec3::ZERO, Vec3::Y),
            ..default()
        },
        BloomSettings::default(),
        MainCamera,
    ))
        .insert(OrbitCameraBundle::new(
            OrbitCameraController {
                mouse_rotate_sensitivity: Vec2::splat(1.),
                mouse_translate_sensitivity: Vec2::splat(30.),
                mouse_wheel_zoom_sensitivity: 0.15,
                smoothing_weight: 0.8,
                enabled: true,
                pixels_per_line: 53.0,
            },
            Vec3::new(-2.5, 4.5, 9.0),
            Vec3::new(0., 0., 0.),
            Vec3::Y,
        ));
}

impl Plugin for CameraControllerPlugin {
    fn build(&self, app: &mut App) {
        app
            .add_plugins((
                LookTransformPlugin,
                OrbitCameraPlugin::default()
            ))
            .add_systems(Startup, setup_camera);
    }
}
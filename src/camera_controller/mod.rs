use bevy::input::mouse::{MouseMotion, MouseWheel};
use bevy::prelude::*;
use bevy::window::PrimaryWindow;
use bevy_inspector_egui::{bevy_egui, egui};
use bevy_inspector_egui::bevy_egui::{EguiContext, EguiContexts};

pub struct CameraControllerPlugin;

#[derive(Component)]
pub struct MainCamera;

#[derive(Component, Reflect)]
pub struct CameraPanning {
    is_panning: bool,
}

#[derive(Component, Reflect)]
struct CameraZoom {
    value: f32,
}

#[derive(Resource, Reflect, Default, Debug, Clone)]
struct ZoomControl {
    min_zoom: f32,
    max_zoom: f32,
    zoom_step: f32,
}

fn setup_camera(
    mut commands: Commands,
) {
    commands.spawn(Camera2dBundle::default())
        .insert(MainCamera)
        .insert(CameraPanning { is_panning: false })
        .insert(CameraZoom { value: 1.0 });
}

fn handle_mouse_input(
    mut buttons: Res<Input<MouseButton>>,
    mut query: Query<&mut CameraPanning>,
) {
    for mut camera_panning in query.iter_mut() {
        camera_panning.is_panning = buttons.pressed(MouseButton::Middle)
    }
}


fn pan_camera_system(
    mut ev_mouse_motion: EventReader<MouseMotion>,
    mut query: Query<(&CameraPanning, &mut Transform)>,
) {
    for (camera_panning, mut transform) in query.iter_mut() {
        if camera_panning.is_panning {
            for event in ev_mouse_motion.iter() {
                let translation = &mut transform.translation;
                translation.x -= event.delta.x;
                translation.y += event.delta.y;
            }
        }
    }
}

fn zoom_camera_system(
    mut ev_mouse_wheel: EventReader<MouseWheel>,
    zoom_control: Res<ZoomControl>,
    mut query: Query<(&mut CameraZoom, &mut Transform)>,
    mut contexts: EguiContexts
) {
    if contexts.ctx().wants_pointer_input() {
        return;
    }

    for event in ev_mouse_wheel.iter() {
        for (mut camera_zoom, mut transform) in query.iter_mut() {
            let zoom_change = if event.y > 0.0 {
                -zoom_control.zoom_step
            } else if event.y < 0.0 {
                zoom_control.zoom_step
            } else {
                0.0
            };

            camera_zoom.value = (camera_zoom.value + zoom_change)
                .clamp(zoom_control.min_zoom, zoom_control.max_zoom);

            transform.scale = Vec3::splat(camera_zoom.value);
        }
    }
}


impl Plugin for CameraControllerPlugin {
    fn build(&self, app: &mut App) {
        app
            .register_type::<CameraZoom>()
            .register_type::<ZoomControl>()
            .register_type::<CameraPanning>()
            .insert_resource(ZoomControl {
                min_zoom: 0.1,
                max_zoom: 2.0,
                zoom_step: 0.1,
            })
            .add_systems(Startup,setup_camera)
            .add_systems(Update, (handle_mouse_input, zoom_camera_system, pan_camera_system));
    }
}
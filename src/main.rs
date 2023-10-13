use bevy::{prelude::*};
use bevy::input::ButtonState;
use bevy::input::mouse::{MouseButtonInput, MouseMotion};
use bevy_cursor::{CursorInfo, CursorInfoPlugin};
use bevy_inspector_egui::InspectorOptions;
use bevy_inspector_egui::quick::{ResourceInspectorPlugin, WorldInspectorPlugin};
use test_plugin::TestPlugin;

fn main() {
    App::new()
        .add_plugins((
            DefaultPlugins,
            WorldInspectorPlugin::new(),
            TestPlugin,
            CursorInfoPlugin
        ))
        .register_type::<CameraPanning>()
        .add_systems(Startup, setup_camera)
        .add_systems(Update, handle_mouse_input)
        .add_systems(Update, pan_camera_system)
        .add_systems(Update, print_cursor_position)
        .run();
}

#[derive(Component, Reflect, InspectorOptions)]
struct CameraPanning {
    is_panning: bool,
}

fn setup_camera(
    mut commands: Commands,
) {
    commands.spawn(Camera2dBundle::default())
        .insert(CameraPanning { is_panning: false });
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


fn print_cursor_position(cursor: Res<CursorInfo>) {
    if let Some(position) = cursor.position() {
        info!("Cursor position: {position:?}");
    } else {
        info!("The cursor is not in any window");
    }
}

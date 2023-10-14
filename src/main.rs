use bevy::{prelude::*};
use bevy::window::PresentMode;
use bevy_cursor::{CursorInfoPlugin};
use crate::camera_controller::CameraControllerPlugin;
use crate::chase::ChasePlugin;
use crate::debug_ui::DebugUIPlugin;

mod camera_controller;
mod chase;
mod debug_ui;

fn main() {
    App::new()
        .insert_resource(ClearColor(Color::BLACK))
        .add_plugins((
            DefaultPlugins.set(WindowPlugin {
                primary_window: Some(Window {
                    title: "Proc Gen".into(),
                    present_mode: PresentMode::AutoNoVsync,
                    resolution: (1280., 720.).into(),
                    ..default()
                }),
                ..default()
            }),
            CursorInfoPlugin,
            DebugUIPlugin,
            CameraControllerPlugin,
            ChasePlugin
        ))
        .run();
}
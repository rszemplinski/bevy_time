use bevy::{prelude::*};
use bevy::window::PresentMode;
use bevy_cursor::{CursorInfoPlugin};
use bevy_xpbd_3d::prelude::PhysicsPlugins;
use crate::camera_controller::CameraControllerPlugin;
use crate::solar_system::ChasePlugin;
use crate::debug_ui::DebugUIPlugin;
use crate::universe::Universe;

mod camera_controller;
mod solar_system;
mod debug_ui;
mod universe;

fn main() {
    App::new()
        .insert_resource(ClearColor(Color::BLACK))
        .init_resource::<Universe>()
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
            PhysicsPlugins::default(),
            CursorInfoPlugin,
            DebugUIPlugin,
            CameraControllerPlugin,
            ChasePlugin
        ))
        .run();
}
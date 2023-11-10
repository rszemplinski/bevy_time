use bevy::{prelude::*};
use bevy::window::PresentMode;
use bevy_cursor::{CursorInfoPlugin};
use bevy_infinite_grid::{InfiniteGridBundle, InfiniteGridPlugin, InfiniteGridSettings};
use bevy_mod_picking::DefaultPickingPlugins;
use bevy_xpbd_3d::prelude::PhysicsPlugins;
use crate::camera_controller::CameraControllerPlugin;
use crate::solar_system::SolarSystemPlugin;
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
                    ..default()
                }),
                ..default()
            }),
            DefaultPickingPlugins,
            PhysicsPlugins::default(),
            CursorInfoPlugin,
            InfiniteGridPlugin,
            DebugUIPlugin,
            CameraControllerPlugin,
            SolarSystemPlugin
        ))
        .add_systems(Startup, infinite_grid_setup)
        .run();
}

fn infinite_grid_setup(mut commands: Commands) {
    commands.spawn(InfiniteGridBundle {
        settings: InfiniteGridSettings {
            shadow_color: None,
            ..default()
        },
        ..default()
    });
}
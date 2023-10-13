use bevy::{prelude::*};
use bevy::core::{Name};
use bevy::sprite::MaterialMesh2dBundle;
use bevy_cursor::{CursorInfoPlugin};
use bevy_inspector_egui::bevy_egui::EguiPlugin;
use bevy_inspector_egui::DefaultInspectorConfigPlugin;
use bevy_inspector_egui::quick::{WorldInspectorPlugin};
use crate::camera_controller::CameraControllerPlugin;
use crate::chase::ChasePlugin;

mod camera_controller;
mod chase;

fn main() {
    App::new()
        .add_plugins((
            DefaultPlugins,
            EguiPlugin,
            DefaultInspectorConfigPlugin,
            CursorInfoPlugin,
            CameraControllerPlugin,
            ChasePlugin
        ))
        .run();
}
use bevy::diagnostic::{DiagnosticsStore, EntityCountDiagnosticsPlugin, FrameTimeDiagnosticsPlugin};
use bevy::input::ButtonState;
use bevy::input::keyboard::KeyboardInput;
use bevy::prelude::*;
use bevy_egui::{egui, EguiContexts, EguiPlugin, EguiSet};
use bevy_inspector_egui::DefaultInspectorConfigPlugin;

pub struct DebugUIPlugin;

fn display_debug_stats(mut egui: EguiContexts, diagnostics: Res<DiagnosticsStore>) {
    egui::Window::new("Performance").show(egui.ctx_mut(), |ui| {
        ui.label(format!(
            "Avg. FPS: {:.02}",
            diagnostics
                .get(FrameTimeDiagnosticsPlugin::FPS)
                .unwrap()
                .average()
                .unwrap_or_default()
        ));
        ui.label(format!(
            "Total Entity count: {}",
            diagnostics
                .get(EntityCountDiagnosticsPlugin::ENTITY_COUNT)
                .unwrap()
                .value()
                .unwrap_or_default()
        ));
    });
}

fn toggle_debug_ui_displays(
    mut inputs: EventReader<KeyboardInput>,
    mut ui_state: ResMut<DebugUIState>,
) {
    for input in inputs.read() {
        match input.key_code {
            Some(key_code) if key_code == KeyCode::F3 && input.state == ButtonState::Pressed => {
                ui_state.display_debug_info = !ui_state.display_debug_info;
            }
            _ => {}
        }
    }
}

fn display_debug_ui_criteria(ui_state: Res<DebugUIState>) -> bool {
    ui_state.display_debug_info
}

#[derive(Clone, Copy, PartialEq, Eq, Hash, Debug, SystemSet)]
/// Systems related to the debug UIs.
pub enum DebugUISet {
    Toggle,
    Display,
}

impl Plugin for DebugUIPlugin {
    fn build(&self, app: &mut App) {
        app
            .add_plugins((
                // LogDiagnosticsPlugin::default(),
                FrameTimeDiagnosticsPlugin,
                EntityCountDiagnosticsPlugin,
                EguiPlugin,
                DefaultInspectorConfigPlugin,
            ))
            .add_systems(
                Update,
                toggle_debug_ui_displays.in_set(DebugUISet::Toggle))
            .add_systems(
                Update,
                display_debug_stats
                    .in_set(DebugUISet::Display)
                    .run_if(display_debug_ui_criteria),
            )
            .configure_sets(Update, (DebugUISet::Toggle, DebugUISet::Display)
                .chain().before(EguiSet::ProcessInput))
            .insert_resource(DebugUIState {
                display_debug_info: true
            });
    }
}

#[derive(Resource, Default)]
struct DebugUIState {
    display_debug_info: bool,
}
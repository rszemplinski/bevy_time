use bevy::diagnostic::{DiagnosticsStore, EntityCountDiagnosticsPlugin, FrameTimeDiagnosticsPlugin};
use bevy::input::common_conditions::input_toggle_active;
use bevy::prelude::*;
use bevy_egui::{egui, EguiContexts, EguiPlugin};
use bevy_inspector_egui::DefaultInspectorConfigPlugin;

pub struct DebugUIPlugin;

fn display_debug_stats(mut egui: EguiContexts, diagnostics: Res<DiagnosticsStore>) {
    egui::Window::new("Performance")
        .default_size(egui::vec2(200., 50.))
        .show(egui.ctx_mut(), |ui| {
            egui::ScrollArea::vertical().show(ui, |ui| {
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
                ui.allocate_space(ui.available_size());
            });
        });
}

impl Plugin for DebugUIPlugin {
    fn build(&self, app: &mut App) {
        app
            .add_plugins((
                FrameTimeDiagnosticsPlugin,
                EntityCountDiagnosticsPlugin,
                EguiPlugin,
                DefaultInspectorConfigPlugin,
            ))
            .add_systems(
                Update,
                display_debug_stats
                    .run_if(input_toggle_active(true, KeyCode::F3)),
            );
    }
}
use std::time::Duration;
use bevy::diagnostic::{DiagnosticsStore, EntityCountDiagnosticsPlugin, FrameTimeDiagnosticsPlugin};
use bevy::input::common_conditions::input_toggle_active;
use bevy::prelude::*;
use bevy::time::common_conditions::on_timer;
use bevy_egui::{egui, EguiContexts, EguiPlugin};
use bevy_inspector_egui::DefaultInspectorConfigPlugin;

pub struct DebugUIPlugin;

fn display_debug_stats(
    time: Res<Time>,
    mut egui: EguiContexts,
    mut timer: ResMut<DebugStatsTimer>,
    diagnostics: Res<DiagnosticsStore>,
    mut debug_stats: Local<DebugStats>,
) {
    timer.0.tick(time.delta());

    if timer.0.just_finished() {
        debug_stats.avg_fps = diagnostics
            .get(FrameTimeDiagnosticsPlugin::FPS)
            .and_then(|fps| fps.smoothed())
            .unwrap_or_default();

        debug_stats.current_fps = diagnostics
            .get(FrameTimeDiagnosticsPlugin::FPS)
            .and_then(|fps| fps.value())
            .unwrap_or_default();

        debug_stats.entity_count = diagnostics
            .get(EntityCountDiagnosticsPlugin::ENTITY_COUNT)
            .and_then(|count| count.value())
            .unwrap_or_default();
    }

    egui::Window::new("Performance")
        .default_size(egui::vec2(200., 50.))
        .show(egui.ctx_mut(), |ui| {
            egui::ScrollArea::vertical().show(ui, |ui| {
                ui.label(format!("FPS: {:.2}", debug_stats.current_fps));
                ui.label(format!("Avg. FPS: {:.2}", debug_stats.avg_fps));
                ui.label(format!("Total Entity count: {}", debug_stats.entity_count));
                ui.allocate_space(ui.available_size());
            });
        });
}

#[derive(Resource, Debug)]
struct DebugStatsTimer(Timer);

#[derive(Default)]
struct DebugStats {
    avg_fps: f64,
    current_fps: f64,
    entity_count: f64,
}

impl Plugin for DebugUIPlugin {
    fn build(&self, app: &mut App) {
        app
            .insert_resource(DebugStatsTimer(Timer::from_seconds(0.5, TimerMode::Repeating)))
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
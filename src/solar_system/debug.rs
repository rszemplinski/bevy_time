use std::marker::PhantomData;
use std::sync::Mutex;
use bevy::ecs::schedule::BoxedCondition;
use bevy::prelude::*;
use bevy::window::PrimaryWindow;
use bevy_egui::EguiContext;
use bevy_inspector_egui::{bevy_inspector, egui};
use crate::solar_system::celestial::CelestialBody;

pub struct CelestialBodyDebugPlugin {
    condition: Mutex<Option<BoxedCondition>>,
    marker: PhantomData<fn()>,
}

impl Default for CelestialBodyDebugPlugin {
    fn default() -> Self {
        Self {
            condition: Mutex::new(None),
            marker: PhantomData,
        }
    }
}

impl CelestialBodyDebugPlugin {
    /// Only show the UI of the specified condition is active
    pub fn run_if<M>(mut self, condition: impl Condition<M>) -> Self {
        let condition_system = IntoSystem::into_system(condition);
        self.condition = Mutex::new(Some(Box::new(condition_system) as BoxedCondition));
        self
    }
}

impl Plugin for CelestialBodyDebugPlugin
{
    fn build(&self, app: &mut App) {
        let condition: Option<Box<dyn ReadOnlySystem<In=(), Out=bool>>> =
            self.condition.lock().unwrap().take();
        let mut system = celestial_debug.into_configs();
        if let Some(condition) = condition {
            system.run_if_dyn(condition);
        }
        app.add_systems(Update, system);
    }
}

fn celestial_debug(world: &mut World) {
    let ctx = world
        .query_filtered::<&mut EguiContext, With<PrimaryWindow>>()
        .get_single(world);

    let Ok(ctx) = ctx else {
        return;
    };
    let mut ctx = ctx.clone();
    let rect = ctx.get_mut().available_rect();

    egui::Window::new("Celestial Bodies")
        .pivot(egui::Align2::RIGHT_TOP)
        .default_pos(egui::pos2(rect.size().x - 20., 20.))
        .default_size(egui::vec2(200., 250.))
        .show(ctx.get_mut(), |ui| {
            egui::ScrollArea::vertical().show(ui, |ui| {
                bevy_inspector::ui_for_world_entities_filtered::<With<CelestialBody>>(world, ui, true);
                ui.allocate_space(ui.available_size());
            });
        });
}
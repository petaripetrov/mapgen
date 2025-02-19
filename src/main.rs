mod mapgen;
mod state;

use bevy::{prelude::*, window::PrimaryWindow};

use bevy_inspector_egui::{
    bevy_egui::{EguiContext, EguiPlugin},
    egui,
};
use mapgen::{MapgenPlugin, MapgenSettings};
use serde::{Deserialize, Serialize};
use state::RegenCells;

#[derive(Default, States, Serialize, Deserialize, Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum DemoState {
    #[default]
    Renderer,
    Mapgen,
}

fn main() {
    App::new()
        .add_plugins((
            DefaultPlugins,
            EguiPlugin,
            MapgenPlugin,
            bevy_inspector_egui::DefaultInspectorConfigPlugin,
        ))
        .init_state::<DemoState>()
        .add_event::<RegenCells>()
        .add_systems(Update, inspector_ui)
        .run();
}

fn inspector_ui(world: &mut World) {
    let Ok(egui_context) = world
        .query_filtered::<&mut EguiContext, With<PrimaryWindow>>()
        .get_single(world)
    else {
        return;
    };
    let mut egui_context = egui_context.clone();

    egui::Window::new("UI").show(egui_context.get_mut(), |ui| {
        egui::ScrollArea::vertical().show(ui, |ui| {
            ui.heading("Resources");
            ui.horizontal(|ui| {
                bevy_inspector_egui::bevy_inspector::ui_for_resource::<MapgenSettings>(world, ui);
                if ui.button("Regenerate Cells").clicked() {
                    world.send_event_default::<RegenCells>();
                }
            });
        });
    });
}

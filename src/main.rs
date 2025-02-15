mod camera;
mod ui;

mod mapgen;
mod renderers;

use bevy::{prelude::*, window::PrimaryWindow};

use bevy_inspector_egui::{bevy_egui::{EguiContext, EguiPlugin}, egui};
use mapgen::MapgenPlugin;
use renderers::RenderersPlugin;
use serde::{Deserialize, Serialize};
use ui::UIPlugin;

#[derive(Default, States, Serialize, Deserialize, Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum DemoState {
    #[default]
    Renderer,
    Mapgen
}

fn main() {
    App::new()
        .add_plugins((
            DefaultPlugins,
            UIPlugin,
            EguiPlugin,
            RenderersPlugin,
            MapgenPlugin,
            bevy_inspector_egui::DefaultInspectorConfigPlugin,
            
        ))
        .init_state::<DemoState>()
        .add_systems(Update, (inspector_ui))
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
            // equivalent to `WorldInspectorPlugin`
            // bevy_inspector_egui::bevy_inspector::ui_for_world(world, ui);

            // egui::CollapsingHeader::new("Materials").show(ui, |ui| {
            //     let test = query.get_single();

            //     if let Ok(entity) = test {
            //         // bevy_inspector_egui::bevy_inspector::ui_for_entities(world, &[entity], ui);
            //     }
            // });

            ui.heading("Entities");
            bevy_inspector_egui::bevy_inspector::ui_for_entities(world, ui);
        });
    });
}

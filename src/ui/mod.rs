/**
 * TODO
 *  - Make UI state serializable
 */
use bevy::{
    app::{Plugin, Update},
    prelude::Commands,
};
use bevy_egui::{
    egui, EguiContexts, EguiPlugin
};

#[derive(Default, Debug, Clone, PartialEq, Eq, Hash)]
enum RendererState {
    #[default]
    Basic,
    Toon,
    PBR,
}

#[derive(Default)]
struct UIState {
    drag_control: f32,
    renderer: RendererState,
    color: [u8; 3], // replace this with a Material struct that holds all of the
                    // necessary material properties
                    // and that gets shipped off to the GPU 
}

pub struct UIPlugin;

impl Plugin for UIPlugin {
    fn build(&self, app: &mut bevy::prelude::App) {
        // Init EGUI
        app.add_plugins(EguiPlugin);

        let mut ui_state = UIState {
            drag_control: 1.0,
            ..Default::default()
        };

        let ui_closure = move |ui: EguiContexts, cmd: Commands| spawn_ui(ui, cmd, &mut ui_state);

        app.add_systems(Update, ui_closure);
    }
}

// EguiContexts is an alias for a Query type
fn spawn_ui(mut egui_context: EguiContexts, mut _commands: Commands, egui_state: &mut UIState) {
    if let Some(context) = egui_context.try_ctx_mut() {
        egui::Window::new("Render Controls")
            .vscroll(false)
            .resizable(true)
            .show(context, |ui| {
                egui::ComboBox::from_label("Renderer")
                    .selected_text(format!("{:?}", egui_state.renderer))
                    .show_ui(ui, |ui| {
                        ui.selectable_value(
                            &mut egui_state.renderer,
                            RendererState::Basic,
                            "Basic",
                        );
                        ui.selectable_value(&mut egui_state.renderer, RendererState::Toon, "Toon");
                        ui.selectable_value(&mut egui_state.renderer, RendererState::PBR, "PBR");
                    });

                match egui_state.renderer {
                    RendererState::Basic => {
                        ui.horizontal(|ui| {
                            ui.label("Kd value");
                            ui.color_edit_button_srgb(&mut egui_state.color);
                        });

                        ui.add(
                            egui::DragValue::new(&mut egui_state.drag_control)
                                .speed(1.0)
                                .range(-5.0..=5.0),
                        );
                    }
                    RendererState::Toon => {}
                    RendererState::PBR => {}
                }

                ui.allocate_space(ui.available_size());
            });
    }
}

mod camera;
mod ui;

mod mapgen;
mod renderers;

use bevy::prelude::*;

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
            RenderersPlugin,
            MapgenPlugin
        ))
        // .init_state::<DemoState>()
        .run();
}

mod ui;

use bevy::prelude::*;
use ui::UIPlugin;

fn main() {
    App::new()
        .add_plugins((DefaultPlugins, UIPlugin))
        .run();
}

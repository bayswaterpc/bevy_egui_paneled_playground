mod app_state;
mod bevy_minigames;
mod egui_ui;

use crate::app_state::AppState;
use crate::bevy_minigames::breakout::add_breakout;
use crate::bevy_minigames::contributors::add_contributors;
use crate::bevy_minigames::snake::add_snake;
use crate::egui_ui::egui_ui;
use bevy::prelude::*;
use bevy_egui::EguiPlugin;

/// An implementation of the classic game "Breakout" with egui panels
fn main() {
    let mut app = AppBuilder::default();
    //.insert_resource(ClearColor(Color::rgb(0.04, 0.04, 0.04)))
    app.insert_resource(WindowDescriptor {
        title: "Egui Bevy Paneled Playground".to_string(),
        ..Default::default()
    })
    .insert_resource(ClearColor(Color::rgb(0.04, 0.04, 0.04)))
    .add_plugins(DefaultPlugins)
    .add_plugin(EguiPlugin)
    .add_state(AppState::CentralPanelState)
    .add_system(egui_ui.system());

    add_breakout(&mut app);
    add_contributors(&mut app);
    add_snake(&mut app);
    app.run();
}

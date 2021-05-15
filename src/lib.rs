mod app_state;
mod bevy_minigames;
mod egui_ui;

use crate::app_state::AppState;
use crate::bevy_minigames::breakout::add_breakout;
use crate::bevy_minigames::contributors::add_contributors;
use crate::bevy_minigames::snake::add_snake;
use crate::egui_ui::add_egui_example;
use bevy::prelude::*;
use bevy_egui::EguiPlugin;

pub fn run_app_main() {
    let mut app = AppBuilder::default();
    //.insert_resource(ClearColor(Color::rgb(0.04, 0.04, 0.04)))
    app.insert_resource(WindowDescriptor {
        title: "Egui Bevy Paneled Playground".to_string(),
        ..Default::default()
    })
    .insert_resource(ClearColor(Color::rgb(0.04, 0.04, 0.04)))
    .add_plugins(DefaultPlugins)
    .add_plugin(EguiPlugin)
    .add_state(AppState::EguiCenter);

    add_egui_example(&mut app);
    add_breakout(&mut app);
    add_contributors(&mut app);
    add_snake(&mut app);
    app.run();
}

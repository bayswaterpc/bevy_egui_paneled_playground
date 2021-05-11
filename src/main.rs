mod app_state;
mod bevy_minigames;

use bevy_egui::{egui, EguiContext, EguiPlugin};
use bevy::{
    prelude::*,
};
use crate::app_state::AppState;
use crate::bevy_minigames::breakout::add_breakout_systems;



/// An implementation of the classic game "Breakout" with egui panels
fn main() {
    let mut app_builder = AppBuilder::default();
    app_builder
        .add_plugins(DefaultPlugins)
        .add_plugin(EguiPlugin)
        .add_state(AppState::CentralPanelState)
        .add_startup_system(setup_cameras.system())
        .add_system(ui_egui.system());

    add_breakout_systems(&mut app_builder);
    
    app_builder.run();
}



fn setup_cameras(mut commands: Commands) {
    // cameras
    commands.spawn_bundle(OrthographicCameraBundle::new_2d());
    commands.spawn_bundle(UiCameraBundle::default());
}


fn ui_egui(
    mut app_state: ResMut<State<AppState>>,
    egui_context: Res<EguiContext>,
) {
    egui::SidePanel::left("side_panel", 200.0).show(egui_context.ctx(), |ui| {
        ui.heading("Side Panel");
        if ui.button("Central Panel").clicked() {
            match app_state.set(AppState::CentralPanelState) {
                Ok(_) => {},
                Err(e) => {println!("{:?}", e)},
            }
        }
        if ui.button("Game Panel").clicked() {
            match app_state.set(AppState::GameState) {
                Ok(_) => {},
                Err(e) => {println!("{:?}", e)},
            }
        }
        if ui.button("Console Log").clicked() {
            println!("app_state {:?}", app_state.current());
        }
    });

    match app_state.current().clone() {
        AppState::CentralPanelState => {
            egui_center_panel(app_state, egui_context);
        },
        _ => return,
    }

}

fn egui_center_panel(
    app_state: ResMut<State<AppState>>,
    egui_context: Res<EguiContext>,
) {

    egui::CentralPanel::default().show(egui_context.ctx(), |ui| {
        ui.heading("Central Panel");
        if ui.button("Print App State Log").clicked() {
            println!("app_state {:?}", app_state.current());
        }
    });
}
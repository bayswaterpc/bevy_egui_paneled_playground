use crate::app_state::AppState;
use bevy::prelude::*;
use bevy_egui::{egui, EguiContext};

pub fn egui_ui(mut app_state: ResMut<State<AppState>>, egui_context: Res<EguiContext>) {
    egui::SidePanel::left("side_panel", 200.0).show(egui_context.ctx(), |ui| {
        ui.heading("Side Panel");
        if ui.button("Central Panel").clicked() {
            match app_state.set(AppState::CentralPanelState) {
                Ok(_) => {}
                Err(e) => {
                    println!("{:?}", e)
                }
            }
        }
        if ui.button("Breakout").clicked() {
            match app_state.set(AppState::Breakout) {
                Ok(_) => {}
                Err(e) => {
                    println!("{:?}", e)
                }
            }
        }
        if ui.button("Contributors").clicked() {
            match app_state.set(AppState::Contributors) {
                Ok(_) => {}
                Err(e) => {
                    println!("{:?}", e)
                }
            }
        }
        if ui.button("Snake").clicked() {
            match app_state.set(AppState::Snake) {
                Ok(_) => {}
                Err(e) => {
                    println!("{:?}", e)
                }
            }
        }
        if ui.button("Console Log").clicked() {
            println!("app_state {:?}", app_state.current());
        }
    });

    match app_state.current().clone() {
        AppState::CentralPanelState => {
            egui_center_panel(app_state, egui_context);
        }
        _ => return,
    }
}

fn egui_center_panel(app_state: ResMut<State<AppState>>, egui_context: Res<EguiContext>) {
    egui::CentralPanel::default().show(egui_context.ctx(), |ui| {
        ui.heading("Central Panel");
        if ui.button("Print App State Log").clicked() {
            println!("app_state {:?}", app_state.current());
        }
    });
}

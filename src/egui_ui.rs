use crate::app_state::AppState;
use bevy::prelude::*;
use bevy_egui::{egui, EguiContext};

#[derive(Clone, Eq, PartialEq, Hash, Debug)]
pub struct EguiState {
    show_dynamic_left_button: bool,
    show_center_message: bool,
    center_input_line: String,
}

fn egui_ui(
    mut app_state: ResMut<State<AppState>>,
    mut egui_state: ResMut<EguiState>,
    egui_context: Res<EguiContext>,
) {
    egui::SidePanel::left("side_panel", 200.0).show(egui_context.ctx(), |ui| {
        ui.heading("Side Panel");
        if ui.button("Egui").clicked() {
            match app_state.set(AppState::EguiCenter) {
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
        if ui.button("Dynamic").clicked() {
            egui_state.show_dynamic_left_button = true;
        }

        if egui_state.show_dynamic_left_button {
            if ui.button("Buttons").clicked() {
                egui_state.show_dynamic_left_button = false;
            }
        }
    });

    match app_state.current().clone() {
        AppState::EguiCenter => {
            egui_center_panel(egui_state, egui_context);
        }
        _ => return,
    }
}

fn egui_center_panel(mut egui_state: ResMut<EguiState>, egui_context: Res<EguiContext>) {
    egui::CentralPanel::default().show(egui_context.ctx(), |ui| {
        ui.heading("Central Panel");
        ui.text_edit_singleline(&mut egui_state.center_input_line);
        if ui.button("Pass Message").clicked() {
            egui_state.show_center_message = !egui_state.show_center_message;
        }
        if egui_state.show_center_message {
            ui.label(format!("Button Says {:?}", egui_state.center_input_line));
        }
    });
}

pub fn add_egui_example(app: &mut AppBuilder) {
    app.insert_resource(EguiState {
        show_dynamic_left_button: false,
        show_center_message: false,
        center_input_line: String::from("wow"),
    })
    .add_system(egui_ui.system());
}


use bevy::prelude::*;
use bevy_egui::{egui, EguiContexts, EguiPlugin};

fn main() {
    App::new()
        .add_plugins(DefaultPlugins)
        .add_plugins(EguiPlugin)
        .add_systems(Update, ui_example)
        .run();
}

fn ui_example(mut ctx: EguiContexts) {
    egui::Window::new("Wits MicroMouse Simulator").show(ctx.ctx_mut(), |ui| {
        ui.label("Hello, World!")
    }

    );
}
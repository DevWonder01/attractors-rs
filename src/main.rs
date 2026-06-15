use egui_macroquad::egui;
use macroquad::prelude::*;

pub mod lorenz;
pub mod preset;
use preset::{Attractor, AttractorKind};

#[macroquad::main("Attractor 3D")]
async fn main() {
    let mut system = Attractor::default_lorenz();
    let mut pos = vec3(0.1, 0.0, 0.0);
    let mut path: Vec<Vec3> = Vec::new();

    let mut camera = Camera3D {
        position: vec3(0.0, 0.0, 100.0),
        up: vec3(0.0, 1.0, 0.0),
        target: vec3(0.0, 0.0, 0.0),
        ..Default::default()
    };

    let mut auto_rotate = false;
    let mut camera_angle: f32 = 0.0;

    loop {
        clear_background(BLACK);

        for _ in 0..5 {
            let (nx, ny, nz) = system.step(pos.x, pos.y, pos.z, 0.005);
            pos = vec3(nx, ny, nz);
            path.push(pos);
        }

        if path.len() > 5000 {
            path.remove(0);
        }

        if auto_rotate {
            camera_angle += 0.01;
            let radius = (camera.position.x * camera.position.x + camera.position.z * camera.position.z).sqrt();
            let r = if radius == 0.0 { 100.0 } else { radius };
            camera.position.x = r * camera_angle.sin();
            camera.position.z = r * camera_angle.cos();
        } else {
            camera_angle = camera.position.x.atan2(camera.position.z);
        }

        set_camera(&camera);
        for i in 1..path.len() {
            draw_line_3d(path[i - 1], path[i], GREEN);
        }
        set_default_camera();

        egui_macroquad::ui(|egui_ctx| {
            egui::Window::new("Controls").show(egui_ctx, |ui| {
                ui.heading("Select Attractor");
                let mut current_kind = system.kind();
                
                let text = match current_kind {
                    AttractorKind::Lorenz => "Lorenz",
                    AttractorKind::Thomas => "Thomas",
                    AttractorKind::Chen => "Chen",
                    AttractorKind::Dadras => "Dadras",
                    AttractorKind::Rossler => "Rossler",
                    AttractorKind::Halvorsen => "Halvorsen",
                    AttractorKind::RabinovichFabrikant => "Rabinovich-Fabrikant",
                    AttractorKind::ThreeScroll => "Three Scroll",
                    AttractorKind::Sprott => "Sprott",
                    AttractorKind::FourWing => "Four Wing",
                };

                egui::ComboBox::from_label("")
                    .selected_text(text)
                    .show_ui(ui, |ui| {
                        ui.selectable_value(&mut current_kind, AttractorKind::Lorenz, "Lorenz");
                        ui.selectable_value(&mut current_kind, AttractorKind::Thomas, "Thomas");
                        ui.selectable_value(&mut current_kind, AttractorKind::Chen, "Chen");
                        ui.selectable_value(&mut current_kind, AttractorKind::Dadras, "Dadras");
                        ui.selectable_value(&mut current_kind, AttractorKind::Rossler, "Rossler");
                        ui.selectable_value(&mut current_kind, AttractorKind::Halvorsen, "Halvorsen");
                        ui.selectable_value(&mut current_kind, AttractorKind::RabinovichFabrikant, "Rabinovich-Fabrikant");
                        ui.selectable_value(&mut current_kind, AttractorKind::ThreeScroll, "Three Scroll");
                        ui.selectable_value(&mut current_kind, AttractorKind::Sprott, "Sprott");
                        ui.selectable_value(&mut current_kind, AttractorKind::FourWing, "Four Wing");
                    });

                if current_kind != system.kind() {
                    system = match current_kind {
                        AttractorKind::Lorenz => Attractor::default_lorenz(),
                        AttractorKind::Thomas => Attractor::default_thomas(),
                        AttractorKind::Chen => Attractor::default_chen(),
                        AttractorKind::Dadras => Attractor::default_dadras(),
                        AttractorKind::Rossler => Attractor::default_rossler(),
                        AttractorKind::Halvorsen => Attractor::default_halvorsen(),
                        AttractorKind::RabinovichFabrikant => Attractor::default_rf(),
                        AttractorKind::ThreeScroll => Attractor::default_threescroll(),
                        AttractorKind::Sprott => Attractor::default_sprott(),
                        AttractorKind::FourWing => Attractor::default_fourwing(),
                    };
                    path.clear();
                    pos = vec3(0.1, 0.0, 0.0);
                }

                ui.separator();
                ui.heading("Parameters");
                if system.draw_ui(ui) {
                    path.clear();
                }

                ui.separator();
                ui.heading("Camera Position");
                ui.checkbox(&mut auto_rotate, "Auto Rotate");
                ui.add(egui::Slider::new(&mut camera.position.x, -200.0..=200.0).text("Cam X"));
                ui.add(egui::Slider::new(&mut camera.position.y, -200.0..=200.0).text("Cam Y"));
                ui.add(egui::Slider::new(&mut camera.position.z, -200.0..=200.0).text("Cam Z"));
            });
        });

        egui_macroquad::draw();
        next_frame().await;
    }
}

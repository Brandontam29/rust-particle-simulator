#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")] // hide console window on Windows in release

use eframe::egui::{self};

use epaint::{Pos2, Rect};

mod canvas;
mod fast_drop_simulation;
mod particle;

use fast_drop_simulation::FastDropSimulation;

fn main() -> Result<(), eframe::Error> {
    env_logger::init(); // Log to stderr (if you run with `RUST_LOG=debug`).
    let options = eframe::NativeOptions {
        viewport: egui::ViewportBuilder::default().with_inner_size([600.0, 600.0]),
        ..Default::default()
    };

    eframe::run_native(
        "My egui App",
        options,
        Box::new(|cc| {
            // This gives us image support:
            egui_extras::install_image_loaders(&cc.egui_ctx);

            Box::<MyApp>::default()
        }),
    )
}

const WIDTH: f32 = 600.0;
const HEIGHT: f32 = 200.0;

struct MyApp {
    simulation: FastDropSimulation,
}

impl Default for MyApp {
    fn default() -> Self {
        Self {
            simulation: FastDropSimulation::new(
                Pos2 { x: 0.0, y: 30.0 },
                Pos2 {
                    x: WIDTH,
                    y: HEIGHT,
                },
                100,
            ),
        }
    }
}

impl eframe::App for MyApp {
    fn update(&mut self, ctx: &egui::Context, _frame: &mut eframe::Frame) {
        egui::CentralPanel::default().show(ctx, |ui| {
            egui::Image::new(egui::include_image!("../assets/ferris.png"))
                .rounding(5.0)
                .max_size(eframe::egui::Vec2::new(30.0, 30.0))
                .paint_at(
                    ui,
                    Rect {
                        min: Pos2 { x: 0.0, y: 0.0 },
                        max: Pos2 { x: 30.0, y: 30.0 },
                    },
                );
            ui.spacing_mut().indent = 10.0;
            ui.heading("Simulation GUI");

            self.simulation.paint(ui.painter());
            self.simulation.update();
        });
    }
}

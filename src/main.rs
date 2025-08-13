mod image_manager;
mod renderer;
mod tests;

use clap::{Parser};
use eframe::{NativeOptions, egui};

fn main() -> eframe::Result {
    let config = Arguments::parse();

    let native_options = NativeOptions {
        viewport: egui::ViewportBuilder::default()
            .with_inner_size([config.camera_width as f32, config.camera_height as f32])
            .with_taskbar(true)
            .with_fullscreen(true)
        ,

        ..Default::default()
    };
    eframe::run_native(
        "Refraction App",
        native_options,
        Box::new(|cc|
            Ok(Box::new(RefractionApp::new(cc, [config.camera_width as f32, config.camera_height as f32])))
        ))
}

#[derive(Default)]
struct RefractionApp {
    dims: [f32; 2],
    value: bool
}

impl RefractionApp {
    fn new(cc: &eframe::CreationContext<'_>, dims: [f32; 2]) -> Self {
        Self {
            dims,
            value: false
        }
    }
}

impl eframe::App for RefractionApp {
    fn update(&mut self, ctx: &egui::Context, _frame: &mut eframe::Frame) {
        egui::CentralPanel::default().show(ctx, |ui| {
            ui.horizontal(|ui| {
                egui::Frame::canvas(ui.style()).show(ui, |ui| {
                    let (rect, response) =
                        ui.allocate_exact_size([400., 400.].into(), egui::Sense::drag());
                });
            });
            ui.toggle_value(&mut self.value, "Open").clicked();

            if self.value {
                egui::SidePanel::left("my_left_panel").show(ctx, |ui| {
                    ui.label("Hello World!");
                    ui.toggle_value(&mut self.value, "Close");
                });
            }
        });
    }
}

#[derive(Parser, Debug)]
#[command(version, about, long_about = None)]
struct Arguments {
    #[arg(long = "width", default_value_t = 640)]
    camera_width: usize,
    #[arg(long = "height", default_value_t = 480)]
    camera_height: usize,
}

mod app;
mod cell;
mod constants;
mod grid;
mod patterns;
mod rules;

use app::GameApp;
use eframe::egui::ViewportBuilder;

fn main() -> eframe::Result<()> {
    let viewport = ViewportBuilder::default()
        .with_inner_size([1200.0, 800.0])
        .with_title("Conway's Game of Life");
    let native_options = eframe::NativeOptions {
        viewport,
        centered: true,
        ..Default::default()
    };

    eframe::run_native(
        "Conway's Game of Life",
        native_options,
        Box::new(|cc| Ok(Box::new(GameApp::new(cc)))),
    )
}

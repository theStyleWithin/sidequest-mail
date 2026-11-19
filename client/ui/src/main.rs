mod app;
mod demo;
mod style;

fn main() -> eframe::Result {
    let options = eframe::NativeOptions {
        viewport: eframe::egui::ViewportBuilder::default().with_inner_size([1100.0, 700.0]).with_title("Sidequest Mail"),
        ..Default::default()
    };
    eframe::run_native("Sidequest Mail", options, Box::new(|_cc| {
            let mut app = app::GardenApp::new();
            // Optional: start in a given theme, e.g. GARDEN_THEME=fantasy-rpg
            if let Ok(id) = std::env::var("GARDEN_THEME") {
                app.set_theme(&id);
            }
            Ok(Box::new(app))
        }))
}

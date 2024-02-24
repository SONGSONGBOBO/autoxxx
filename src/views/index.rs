use eframe::App;

pub(crate) struct MyEguiApp;

impl Default for MyEguiApp {
    fn default() -> Self {
        Self
    }
}

impl App for MyEguiApp {
    fn update(&mut self, ctx: &egui::Context, _frame: &mut eframe::Frame) {
        egui::CentralPanel::default().show(ctx, |ui| {
            ui.label("Hello, world!");
        });
    }
}
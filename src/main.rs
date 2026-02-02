use eframe::egui;

fn main() {
    let native_options = eframe::NativeOptions::default();
    eframe::run_native(
        "Astropathic Relay",
        native_options,
        Box::new(|cc| Ok(Box::new(AstropathicRelayApp::new(cc)))),
    )
    .unwrap();
}

#[derive(Default)]
struct AstropathicRelayApp {}

impl AstropathicRelayApp {
    fn new(_cc: &eframe::CreationContext<'_>) -> Self {
        Self::default()
    }
}

impl eframe::App for AstropathicRelayApp {
    fn update(&mut self, ctx: &egui::Context, _frame: &mut eframe::Frame) {
        egui::CentralPanel::default().show(ctx, |ui| {
            ui.heading("Astropathic Relay");
            ui.label("Securely connect to devices on your LAN.");
            ui.add_space(10.0)
        });
    }
}

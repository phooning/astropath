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

enum AppState {
    Idle,
    Connecting,
    Connected,
    Error(String),
}

struct AstropathicRelayApp {
    state: AppState,
    target_ip: String,
}

impl AstropathicRelayApp {
    fn new(_cc: &eframe::CreationContext<'_>) -> Self {
        let ip = local_ip_address::local_ip()
            .map(|ip| ip.to_string())
            .unwrap_or_else(|_| "Unknown".to_string());

        Self {
            state: AppState::Idle,
            target_ip: ip,
        }
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

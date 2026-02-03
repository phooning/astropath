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
        Self::new_with_defaults()
    }

    pub fn new_with_defaults() -> Self {
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

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_initial_ui_fields() {
        let app = AstropathicRelayApp::new_with_defaults();

        assert_eq!(
            app.target_ip,
            local_ip_address::local_ip().unwrap().to_string()
        );

        match app.state {
            AppState::Idle => (),
            _ => panic!("Initial state should be Idle"),
        }
    }
}

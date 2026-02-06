use eframe::egui;
use std::time::{Duration, Instant};

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

struct CopyField {
    value: String,
    last_copied: Option<Instant>,
}

impl CopyField {
    fn new(value: &str) -> Self {
        Self {
            value: value.to_string(),
            last_copied: None,
        }
    }

    fn ui(&mut self, ui: &mut egui::Ui) {
        ui.horizontal(|ui| {
            ui.label(&self.value);

            let is_recent = self
                .last_copied
                .map(|t| t.elapsed() < Duration::from_secs(2))
                .unwrap_or(false);

            if ui.button("Copy").clicked() {
                ui.ctx().copy_text(self.value.clone());
                self.last_copied = Some(Instant::now());
            }
        });
    }
}

impl eframe::App for AstropathicRelayApp {
    fn update(&mut self, ctx: &egui::Context, _frame: &mut eframe::Frame) {
        egui::CentralPanel::default().show(ctx, |ui| {
            ui.heading("Astropathic Relay");
            ui.label("Securely connect to devices on your LAN.");
            ui.add_space(10.0);

            if let Ok(ip) = local_ip_address::local_ip() {
                ui.horizontal(|ui| {
                    ui.label("Your Local IP Address:");
                    ui.label(
                        egui::RichText::new(format!("{}", ip))
                            .strong()
                            .color(egui::Color32::LIGHT_BLUE),
                    );

                    if ui.button("Copy").clicked() {
                        ui.ctx().copy_text(ip.to_string());
                    }
                });
            } else {
                ui.label("Unable to determine local IP address.");
            }
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

        let expected_ip = local_ip_address::local_ip().unwrap().to_string();
        assert_eq!(
            app.target_ip, expected_ip,
            "Target IP should match local IP address."
        );
    }
}

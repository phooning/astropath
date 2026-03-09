mod net;

use eframe::egui;
use std::time::{Duration, Instant};
use tokio::{runtime::Runtime, sync::mpsc};

use crate::net::NetEvent;

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

const DEFAULT_PORT: u16 = 9001;

struct AstropathicRelayApp {
    rt: Runtime,
    state: AppState,
    port: String,
    target_ip: CopyField,
    tailscale_ip: Option<CopyField>,
    receiver: mpsc::Receiver<NetEvent>,
    transmitter: mpsc::Sender<NetEvent>,
}

impl AstropathicRelayApp {
    fn new(_cc: &eframe::CreationContext<'_>) -> Self {
        Self::new_with_defaults()
    }

    pub fn new_with_defaults() -> Self {
        let ip = local_ip_address::local_ip()
            .map(|ip| ip.to_string())
            .unwrap_or_else(|_| "Unknown".to_string());

        let mut tailscale_ip_str = None;
        if let Ok(interfaces) = local_ip_address::list_afinet_netifas() {
            for (name, if_ip) in interfaces {
                let name_lower = name.to_lowercase();
                if name_lower.contains("tailscale") {
                    tailscale_ip_str = Some(if_ip.to_string());
                    break;
                }
                if let std::net::IpAddr::V4(ipv4) = if_ip {
                    let octets = ipv4.octets();
                    // CGNAT range 100.64.0.0/10 used by Tailscale
                    if octets[0] == 100 && (64..=127).contains(&octets[1]) {
                        tailscale_ip_str = Some(if_ip.to_string());
                    }
                }
            }
        }

        let rt = tokio::runtime::Builder::new_multi_thread()
            .enable_all()
            .build()
            .unwrap();

        let (tx, rx): (mpsc::Sender<NetEvent>, mpsc::Receiver<NetEvent>) = mpsc::channel(100);

        Self {
            rt,
            port: "9001".to_owned(),
            state: AppState::Idle,
            target_ip: CopyField::new(&ip),
            tailscale_ip: tailscale_ip_str.map(|s| CopyField::new(&s)),
            receiver: rx,
            transmitter: tx,
        }
    }

    fn start_host(&mut self) {
        let port = self.port.parse::<u16>().unwrap_or(DEFAULT_PORT);
        let tx = self.transmitter.clone();

        self.rt.spawn(async move {
            let addr = format!("0.0.0.0:{}", port);
            match tokio::net::TcpListener::bind(&addr).await {
                Ok(listener) => loop {
                    match listener.accept().await {
                        Ok((_stream, client_addr)) => {
                            let _ = tx.send(NetEvent::Connected(client_addr)).await;
                        }
                        Err(e) => {
                            let _ = tx.send(NetEvent::Error(e.to_string())).await;
                        }
                    }
                },
                Err(e) => {
                    let _ = tx.send(NetEvent::Error(e.to_string())).await;
                }
            }
        });

        self.state = AppState::Connecting;
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
            ui.label(
                egui::RichText::new(format!("{}", &self.value))
                    .strong()
                    .color(egui::Color32::LIGHT_BLUE),
            );

            let is_recent = self
                .last_copied
                .map(|t| t.elapsed() < Duration::from_secs(2))
                .unwrap_or(false);

            let label = if is_recent { "Copied" } else { "Copy" };

            if ui.button(label).clicked() {
                ui.ctx().copy_text(self.value.clone());
                self.last_copied = Some(Instant::now());
            }

            // Request repaint in the copied state.
            if is_recent {
                ui.ctx().request_repaint();
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
            ui.separator();

            if local_ip_address::local_ip().is_ok() {
                ui.horizontal(|ui| {
                    ui.label("Your Local IP Address:");
                    self.target_ip.ui(ui);
                });
            } else {
                ui.horizontal(|ui| {
                    ui.label("Your Local IP Address:");
                    ui.label("Unable to determine local IP address.");
                });
            }

            if let Some(ts_ip) = self.tailscale_ip.as_mut() {
                ui.horizontal(|ui| {
                    ui.label("Tailscale IP Address:");
                    ts_ip.ui(ui);
                });
            }

            ui.horizontal(|ui| {
                ui.label("Port:");
                ui.text_edit_singleline(&mut self.port);
            });

            if ui.button("Host (Wait for connection)").clicked() {
                self.start_host();
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
            app.target_ip.value,
            local_ip_address::local_ip().unwrap().to_string(),
        );

        match app.state {
            AppState::Idle => (),
            _ => panic!("Initial state should be Idle"),
        }

        let expected_ip = local_ip_address::local_ip().unwrap().to_string();
        assert_eq!(
            app.target_ip.value, expected_ip,
            "Target IP should match local IP address."
        );
    }
}

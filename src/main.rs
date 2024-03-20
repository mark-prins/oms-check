#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")] // hide console window on Windows in release
mod configuration;
mod environment;
mod setttings;

use eframe::egui;

fn main() -> Result<(), eframe::Error> {
    env_logger::init(); // Log to stderr (if you run with `RUST_LOG=debug`).
    let options = eframe::NativeOptions {
        viewport: egui::ViewportBuilder::default().with_inner_size([320.0, 240.0]),
        ..Default::default()
    };
    eframe::run_native(
        "Open mSupply environment check",
        options,
        Box::new(|cc| {
            // This gives us image support:
            egui_extras::install_image_loaders(&cc.egui_ctx);

            Box::<OmsCheck>::default()
        }),
    )
}

struct OmsCheck {
    url: String,
    username: String,
    error: String,
    configuration: Test,
    connected: Test,
}

struct Test {
    label: &'static str,
    result: Option<TestResult>,
}

#[derive(Clone)]
struct TestResult {
    success: bool,
    error: String,
}

impl Default for OmsCheck {
    fn default() -> Self {
        Self {
            error: "".to_owned(),
            url: "".to_owned(),
            username: "".to_owned(),
            configuration: Test {
                label: "Parsed configuration",
                result: None,
            },
            connected: Test {
                label: "Connect to server",
                result: None,
            },
        }
    }
}

fn show_test(status: &Test, ui: &mut egui::Ui) {
    let (image, message) = match status.result.clone() {
        Some(s) => {
            if s.success {
                (
                    egui::Image::new(egui::include_image!("./assets/check_outline.png")),
                    "Success!".to_owned(),
                )
            } else {
                (
                    egui::Image::new(egui::include_image!("./assets/error_circle.png")),
                    format!("Error:{}", s.error.clone()),
                )
            }
        }
        None => (
            egui::Image::new(egui::include_image!("./assets/help_outline.png")),
            "pending...".to_owned(),
        ),
    };

    ui.horizontal(|ui| {
        ui.add(image.max_width(24.0));
        ui.label(status.label);
        ui.label(message);
    });
}

#[tokio::main]
async fn check_url(url: String, oms_check: &mut OmsCheck) {
    let response = match reqwest::get(url.clone()).await {
        Ok(response) => response,
        Err(e) => {
            oms_check.connected.result = Some(TestResult {
                success: false,
                error: e.to_string(),
            });
            return;
        }
    };

    match response.text().await {
        Ok(_) => {
            oms_check.connected.result = Some(TestResult {
                success: true,
                error: "".to_owned(),
            });
        }
        Err(e) => {
            oms_check.connected.result = Some(TestResult {
                success: false,
                error: e.to_string(),
            });
        }
    }
}

impl eframe::App for OmsCheck {
    fn update(&mut self, ctx: &egui::Context, _frame: &mut eframe::Frame) {
        match configuration::get_configuration() {
            Ok(settings) => {
                self.username = settings.sync.clone().unwrap().username;
                self.url = settings.sync.unwrap().url;
            }
            Err(e) => {
                self.error = e.to_string();
            }
        };

        egui::CentralPanel::default().show(ctx, |ui| {
            ui.heading("Open mSupply environment check");
            if !self.error.is_empty() {
                ui.horizontal(|ui| {
                    ui.add(
                        egui::Image::new(egui::include_image!("./assets/error_circle.png"))
                            .max_width(24.0),
                    );
                    ui.colored_label(
                        egui::Color32::RED,
                        format!("Unable to parse settings: {}", self.error.clone()),
                    );
                });
            } else {
                self.configuration.result = Some(TestResult {
                    success: true,
                    error: "".to_string(),
                });

                ui.vertical(|ui| {
                    ui.label(format!("Sync URL: {}", self.url));
                    ui.end_row();

                    ui.label(format!("Login: {}", self.username));
                    ui.end_row();

                    ui.label("");
                    ui.end_row();
                });

                ui.vertical(|ui| {
                    show_test(&self.configuration, ui);
                    show_test(&self.connected, ui);
                });
                ui.with_layout(egui::Layout::bottom_up(egui::Align::Center), |ui| {
                    if ui.button("Start").clicked() {
                        check_url(self.url.clone(), self);
                    }
                });
            }
        });
    }
}

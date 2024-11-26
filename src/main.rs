#![windows_subsystem = "windows"]
use eframe::egui;

fn main() -> eframe::Result {
    env_logger::init(); // Log to stderr (if you run with `RUST_LOG=debug`).
    let options = eframe::NativeOptions {
        viewport: egui::ViewportBuilder::default().with_inner_size([320.0, 240.0]),
        ..Default::default()
    };
    eframe::run_native("backup_for_zeljko", options, Box::new(|_| Ok(Box::<MyApp>::default())))
}

struct MyApp {
    original_aktivne_datoteke: String,
    backup_1_aktivne_datoteke: String,
    backup_2_aktivne_datoteke: String,
    backup_1_arhivirane_datoteke: String,
    backup_2_arhivirane_datoteke: String,
    files_to_move: Vec<String>,
}

impl Default for MyApp {
    fn default() -> Self {
        // TODO: find dynamically these folders because external drives can have different letters d:, e:, f:
        // but the folder names will remain fixed
        Self {
            original_aktivne_datoteke: r#"d:\aktivne_datoteke"#.to_owned(),
            backup_1_aktivne_datoteke: r#"d:\backup_1\aktivne_datoteke"#.to_owned(),
            backup_2_aktivne_datoteke: r#"d:\backup_2\aktivne_datoteke"#.to_owned(),
            backup_1_arhivirane_datoteke: r#"d:\backup_1\arhivirane_datoteke"#.to_owned(),
            backup_2_arhivirane_datoteke: r#"d:\backup_2\arhivirane_datoteke"#.to_owned(),
            files_to_move: vec![],
        }
    }
}

impl eframe::App for MyApp {
    fn update(&mut self, ctx: &egui::Context, _frame: &mut eframe::Frame) {
        egui::CentralPanel::default().show(ctx, |ui| {
            ui.heading("backup_for_zeljko");
            ui.label(format!("original aktivne_datoteke: {}", self.original_aktivne_datoteke));
            ui.label(format!("primary backup aktivne_datoteke: {}", self.backup_1_aktivne_datoteke));
            ui.label(format!("secondary backup aktivne_datoteke: {}", self.backup_2_aktivne_datoteke));
            ui.label(format!("primary backup arhivirane_datoteke: {}", self.backup_1_arhivirane_datoteke));
            ui.label(format!("secondary backup arhivirane_datoteke: {}", self.backup_2_arhivirane_datoteke));

            if ui.button("Start backup").clicked() {
                // robocopy list only
                    Command::new("cmd")
            .args(&["/C", "echo hello"])
            .output()
            .expect("failed to execute process")
                self.files_to_move.push("xxx".to_owned());
            }
            // TODO: multi line text for files_to_move
        });
    }
}

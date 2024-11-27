#![windows_subsystem = "windows"]
use std::io::BufRead;

use eframe::egui;
use egui::Color32;

fn main() -> eframe::Result {
    env_logger::init(); // Log to stderr (if you run with `RUST_LOG=debug`).
    let options = eframe::NativeOptions {
        viewport: egui::ViewportBuilder::default().with_inner_size([900.0, 600.0]),
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
        // TODO: here find dynamically these folders because external drives can have different letters d:, e:, f:
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
    fn update(&mut self, egui_ctx: &egui::Context, _frame: &mut eframe::Frame) {
        let mut visuals = egui::Visuals::light();
        visuals.widgets.active.fg_stroke.color = Color32::BLACK;
        visuals.override_text_color = Some(Color32::BLACK);
        egui_ctx.set_visuals(visuals);

        let mut fonts = egui::FontDefinitions::default();

        // Install my own font (maybe supporting non-latin characters):
        fonts
            .font_data
            .insert("Roboto-Medium".to_owned(), egui::FontData::from_static(include_bytes!("../fonts/Roboto-Medium.ttf"))); // .ttf and .otf supported

        // Put my font first (highest priority):
        fonts.families.get_mut(&egui::FontFamily::Proportional).unwrap().insert(0, "Roboto-Medium".to_owned());

        // Put my font as last fallback for monospace:
        fonts.families.get_mut(&egui::FontFamily::Monospace).unwrap().push("Roboto-Medium".to_owned());

        egui_ctx.set_fonts(fonts);

        let my_frame = egui::containers::Frame {
            inner_margin: egui::epaint::Margin {
                left: 10.,
                right: 10.,
                top: 10.,
                bottom: 10.,
            },
            fill: egui::Color32::WHITE,
            ..Default::default()
        };

        egui::CentralPanel::default().frame(my_frame).show(egui_ctx, |ui| {
            egui_ctx.set_pixels_per_point(3.0);

            ui.heading("Backup for Željko");
            ui.label(format!("Original 'aktivne': {}", self.original_aktivne_datoteke));
            ui.label(format!("Primary backup 'aktivne': {}", self.backup_1_aktivne_datoteke));
            ui.label(format!("Secondary backup 'aktivne': {}", self.backup_2_aktivne_datoteke));
            ui.label(" ");
            ui.label(format!("Original 'arhivirane': {}", self.backup_1_arhivirane_datoteke));
            ui.label(format!("Backup 'arhivirane': {}", self.backup_2_arhivirane_datoteke));

            if ui.button("Start backup").clicked() {
                self.start_backup_on_click();
            }

            let mut str_file = self.files_to_move.concat();
            ui.text_edit_multiline(&mut str_file);
        });
    }
}

impl MyApp {
    fn start_backup_on_click(&mut self) {
        // robocopy list only
        use std::os::windows::process::CommandExt;
        let output = std::process::Command::new("robocopy")
            .args(&[
                self.original_aktivne_datoteke.clone(),
                self.backup_1_aktivne_datoteke.clone(),
                "/MIR".to_owned(),
                "/L".to_owned(),
                "/X".to_owned(),
                "/FP".to_owned(),
                "/NS".to_owned(),
                "/NC".to_owned(),
                "/NDL".to_owned(),
            ])
            .creation_flags(0x08000000)
            .output()
            .expect("failed to execute process");
        // parse
        for x in output.stdout.lines() {
            self.files_to_move.push(x.unwrap());
        }
    }
}

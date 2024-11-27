// this app is intended just for Windows
#![cfg(target_os = "windows")]
// do not open terminal when executing the program in windows
#![windows_subsystem = "windows"]

use std::{format, vec};

fn main() {
    // scaffolding for catch panic and log to file
    let _log2 = log2::open("log.txt").size(1 * 1024 * 1024).rotate(3).level("debug").start();

    let version: &'static str = env!("CARGO_PKG_VERSION");
    log::info!("Start app backup_for_zeljko v{}", version);

    // catch panics and write to log.txt
    std::panic::set_hook(Box::new(|info| {
        let backtrace = std::backtrace::Backtrace::force_capture();
        handle_panic(info.payload(), backtrace)
    }));
    let _ = std::panic::catch_unwind(|| {
        let _ = main_inner();
    });
}

fn handle_panic(payload: &(dyn std::any::Any + Send), backtrace: std::backtrace::Backtrace) {
    log::error!("Panicked: ");
    if let Some(string) = payload.downcast_ref::<String>() {
        log::error!("{string}");
    } else if let Some(str) = payload.downcast_ref::<&'static str>() {
        log::error!("{str}")
    } else {
        log::error!("{payload:?}")
    }

    log::error!("Backtrace: {backtrace:#?}");
}

fn main_inner() -> eframe::Result {
    let options = eframe::NativeOptions {
        viewport: egui::ViewportBuilder::default().with_inner_size([900.0, 600.0]),
        ..Default::default()
    };
    eframe::run_native("Backup for Željko", options, Box::new(|_| Ok(Box::<MyApp>::default())))
}

struct MyApp {
    original_aktivne_datoteke: Option<String>,
    backup_1_aktivne_datoteke: Option<String>,
    backup_2_aktivne_datoteke: Option<String>,
    original_arhivirane_datoteke: Option<String>,
    backup_arhivirane_datoteke: Option<String>,
    files_changed: Vec<String>,
    backup_is_done: bool,
    count_files_changed: usize,
    text_to_show: String,
}

impl Default for MyApp {
    fn default() -> Self {
        /// internal function
        fn find_exist_folder_in_drives(path_wo_drive_letter: &str) -> Option<String> {
            let drives_letters = vec!["c", "d", "e", "f", "g", "h", "i", "j", "k"];
            for x in drives_letters.iter() {
                let path = format!("{x}{path_wo_drive_letter}");
                if std::fs::exists(&path).unwrap() {
                    return Some(path);
                }
            }
            None
        }

        let original_aktivne_datoteke = r#"d:\aktivne_datoteke"#;
        let original_aktivne_datoteke = if std::fs::exists(original_aktivne_datoteke).unwrap() {
            Some(original_aktivne_datoteke.to_string())
        } else {
            None
        };

        // external drives can have different letters d:, e:, f:,...
        // I need to check where is the foldername I expect
        let backup_1_aktivne_datoteke = find_exist_folder_in_drives(r#":\backup_1\aktivne_datoteke"#);
        let backup_2_aktivne_datoteke = find_exist_folder_in_drives(r#":\backup_2\aktivne_datoteke"#);
        let original_arhivirane_datoteke = find_exist_folder_in_drives(r#":\backup_1\arhivirane_datoteke"#);
        let backup_arhivirane_datoteke = find_exist_folder_in_drives(r#":\backup_2\arhivirane_datoteke"#);

        Self {
            original_aktivne_datoteke,
            backup_1_aktivne_datoteke,
            backup_2_aktivne_datoteke,
            original_arhivirane_datoteke,
            backup_arhivirane_datoteke,
            files_changed: vec![],
            backup_is_done: false,
            count_files_changed: 0,
            text_to_show: "".to_string(),
        }
    }
}

impl eframe::App for MyApp {
    fn update(&mut self, egui_ctx: &egui::Context, _frame: &mut eframe::Frame) {
        // region: change colors
        let mut visuals = egui::Visuals::light();
        visuals.widgets.active.fg_stroke.color = egui::Color32::BLACK;
        visuals.override_text_color = Some(egui::Color32::BLACK);
        egui_ctx.set_visuals(visuals);
        // endregion: change colors

        egui_ctx.all_styles_mut(|style| {
            style.spacing.scroll = egui::style::ScrollStyle::solid();
        });

        // Install my own font (maybe supporting non-latin characters):
        let mut fonts = egui::FontDefinitions::default();
        fonts
            .font_data
            // .ttf and .otf supported
            .insert("Roboto-Medium".to_owned(), egui::FontData::from_static(include_bytes!("../fonts/Roboto-Medium.ttf")));
        // Put my font first (highest priority):
        fonts.families.get_mut(&egui::FontFamily::Proportional).unwrap().insert(0, "Roboto-Medium".to_owned());
        // Put my font as last fallback for monospace:
        fonts.families.get_mut(&egui::FontFamily::Monospace).unwrap().push("Roboto-Medium".to_owned());
        egui_ctx.set_fonts(fonts);

        let my_frame = egui::containers::Frame {
            inner_margin: egui::epaint::Margin {
                left: 10.,
                right: 0.,
                top: 10.,
                bottom: 0.,
            },
            fill: egui::Color32::WHITE,
            ..Default::default()
        };

        egui::CentralPanel::default().frame(my_frame).show(egui_ctx, |ui| {
            egui_ctx.set_pixels_per_point(2.5);
            egui::ScrollArea::vertical()
                .scroll_bar_visibility(egui::scroll_area::ScrollBarVisibility::AlwaysVisible)
                .auto_shrink(false)
                .show(ui, |ui| {
                    ui.heading("Backup for Željko");
                    ui.label(" ");
                    ui.with_layout(egui::Layout::left_to_right(egui::Align::TOP), |ui| {
                        ui.add_enabled_ui(!self.backup_is_done, |ui| {
                            let start_button = ui.button("Start backup");
                            if start_button.clicked() && !self.backup_is_done {
                                self.backup_is_done = true;
                                self.start_all_backups_on_click();
                            }
                        });

                        let exit_button = ui.button("Exit program");
                        if exit_button.clicked() {
                            egui_ctx.send_viewport_cmd(egui::ViewportCommand::Close)
                        }
                    });
                    ui.label(" ");
                    ui.label(format!("Original 'aktivne': {}", self.original_aktivne_datoteke.clone().unwrap_or("".to_string())));
                    ui.label("First backup:");
                    ui.label(format!("---> Primary backup 'aktivne': {}", self.backup_1_aktivne_datoteke.clone().unwrap_or("".to_string())));
                    ui.label("Second backup:");
                    ui.label(format!("---> Secondary backup 'aktivne': {}", self.backup_2_aktivne_datoteke.clone().unwrap_or("".to_string())));
                    ui.label(" ");
                    ui.label(format!("Original 'arhivirane': {}", self.original_arhivirane_datoteke.clone().unwrap_or("".to_string())));
                    ui.label("Third backup:");
                    ui.label(format!("---> Backup 'arhivirane': {}", self.backup_arhivirane_datoteke.clone().unwrap_or("".to_string())));
                    ui.label(" ");
                    ui.label(self.text_to_show.clone());
                });
        });
    }
}

impl MyApp {
    fn start_all_backups_on_click(&mut self) {
        // 3 different backups
        if self.original_aktivne_datoteke.is_some() && self.backup_1_aktivne_datoteke.is_some() {
            self.text_to_show.push_str("First backup\n");
            self.backup(self.original_aktivne_datoteke.clone().unwrap(), self.backup_1_aktivne_datoteke.clone().unwrap());
        }
        if self.original_aktivne_datoteke.is_some() && self.backup_2_aktivne_datoteke.is_some() {
            self.text_to_show.push_str("Second backup\n");
            self.backup(self.original_aktivne_datoteke.clone().unwrap(), self.backup_2_aktivne_datoteke.clone().unwrap());
        }
        if self.original_arhivirane_datoteke.is_some() && self.backup_arhivirane_datoteke.is_some() {
            self.text_to_show.push_str("Third backup\n");
            self.backup(self.original_arhivirane_datoteke.clone().unwrap(), self.backup_arhivirane_datoteke.clone().unwrap());
        }
        self.text_to_show.push_str(&format!("All files changed after backup: {}\n", self.count_files_changed));
    }

    fn backup(&mut self, source: String, destination: String) {
        let output = command_robocopy_list_only(source.clone(), destination.clone());
        self.files_changed = parse_robocopy_output(output);
        self.count_files_changed += self.files_changed.len();
        self.text_to_show.push_str(&self.files_changed.join("\n"));
        self.text_to_show.push('\n');

        // move the files instead of deleting them
        use chrono::{DateTime, Local};
        let current_local: DateTime<Local> = Local::now();
        let now_formatted = current_local.format("%Y-%m-%d_%H-%M-%S").to_string();
        // take the "e:\" part of destination to create the new folder
        let deleted_now_folder = format!("{}zbrisane_datoteke_{now_formatted}", &destination[..3]);
        // let mut debug_vec = vec![];
        for x in &self.files_changed {
            // only the destination folder and prepare to move them
            if x.starts_with(&destination) {
                let move_to = x.replace(&destination, &deleted_now_folder);
                let parent_dir = std::path::Path::new(&move_to).parent().unwrap();
                if !parent_dir.exists() {
                    std::fs::create_dir_all(&parent_dir).unwrap();
                }
                std::fs::rename(x, move_to.clone()).unwrap();
            }
        }
        command_robocopy_mir(source.clone(), destination.clone());
    }
}

/// robocopy list only
fn command_robocopy_list_only(source: String, destination: String) -> std::process::Output {
    // I isolated this call into a function because I need some specific windows flags.
    // That ruins the editor capability to understand what types are used.
    use std::os::windows::process::CommandExt;
    let output = std::process::Command::new("robocopy")
        .args(&[
            source,
            destination,
            "/MIR".to_owned(),
            "/L".to_owned(),
            "/X".to_owned(),
            "/FP".to_owned(),
            "/NS".to_owned(),
            "/NC".to_owned(),
            "/NDL".to_owned(),
        ])
        // specific windows flag to not open the terminal window
        .creation_flags(0x08000000)
        .output()
        .expect("failed to execute process");
    output
}

fn parse_robocopy_output(output: std::process::Output) -> Vec<String> {
    let mut vec_string: Vec<String> = vec![];
    // find the third line ------
    let mut count_del_lines = 0;
    // import the trait that has .lines()
    use std::io::BufRead;
    for x in output.stdout.lines() {
        let x = x.unwrap();
        if x.starts_with("-----") {
            count_del_lines += 1;
        } else if count_del_lines == 3 && !x.is_empty() {
            vec_string.push(x.trim().to_string());
        }
    }
    vec_string
}

/// robocopy MIR
fn command_robocopy_mir(source: String, destination: String) -> std::process::Output {
    // I isolated this call into a function because I need some specific windows flags.
    // That ruins the editor capability to understand what types are used.
    use std::os::windows::process::CommandExt;
    let output = std::process::Command::new("robocopy")
        .args(&[source, destination, "/MIR".to_owned()])
        // specific windows flag to not open the terminal window
        .creation_flags(0x08000000)
        .output()
        .expect("failed to execute process");
    output
}
// rust_analyzer gives here the error: `main` function not found in crate, but it is not true.

// backup_for_zeljko/src/main.rs

#![doc=include_str!("../README.md")]
// This app is intended just for Windows.
#![cfg(target_os = "windows")]
// In VSCode settings#![cfg(target_os = "windows")] I needed to set:
// "rust-analyzer.cargo.target": "x86_64-pc-windows-gnu"
// to avoid rust-analyzer to show me errors that are Linux specific,
// because I cross-compile this program from Linux to Windows.

// Do not open terminal when executing the program in windows
#![windows_subsystem = "windows"]

/// Use unwrap! macro to get the line and column of panics also in release mode.
use unwrap::unwrap;

mod gui_helper;
use gui_helper::*;

static ORIGINAL1: &'static str = r#"original1"#;
static BACKUP1_OF_ORIGINAL1: &'static str = r#"backup1_of_original1"#;
static BACKUP2_OF_ORIGINAL1: &'static str = r#"backup2_of_original1"#;
static ORIGINAL2: &'static str = r#"original2"#;
static BACKUP_OF_ORIGINAL2: &'static str = r#"backup_of_original2"#;

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

/// The original main() is cluttered with standard app scaffolding.
/// Here is only code that is app specific.
fn main_inner() -> iced::Result {
    iced::application("Backup for Željko", MyApp::update, MyApp::view)
        .window(iced::window::Settings {
            size: iced::Size::new(500.0, 600.0),
            ..Default::default()
        })
        .run()
}

/// Messages are basically like user events in the context of "iced".
#[derive(Debug, Clone, Copy)]
enum Message {
    StartBackup,
    ExitProgram,
}

struct MyApp {
    original1: Option<String>,
    backup1_of_original1: Option<String>,
    backup2_of_original1: Option<String>,
    original2: Option<String>,
    backup_of_original2: Option<String>,
    changing_fields: ChangingFields,
}

/// Sub-struct to isolate what fields can change,
/// because of interprocedural-conflicts, keywords: Non-lexical lifetimes (NLL), disjointed capture
/// <https://smallcultfollowing.com/babysteps/blog/2018/11/01/after-nll-interprocedural-conflicts/>
/// <https://smallcultfollowing.com/babysteps/blog/2024/06/02/the-borrow-checker-within/>
struct ChangingFields {
    files_changed: Vec<String>,
    count_files_changed: usize,
    text_to_show: String,
}

impl Default for MyApp {
    /// Defaults are better to show intent then the "constructor" new() method.
    fn default() -> Self {
        /// Internal function, because it is called only once and only here.
        fn find_exist_folder_in_drives(path_wo_drive_letter: &str) -> Option<String> {
            let drives_letters = vec!["c", "d", "e", "f", "g", "h", "i", "j", "k", "l", "m", "n", "o", "p", "q", "r", "s", "t", "u", "v", "w", "x", "y", "z"];
            for x in drives_letters.iter() {
                let path = format!(r#"{x}:\{path_wo_drive_letter}"#);
                if std::fs::exists(&path).unwrap() {
                    return Some(path);
                }
            }
            None
        }

        // External drives can have different letters d:, e:, f:,...
        // I need to check where is the foldername I expect. The folder names are fixed.
        let original1 = find_exist_folder_in_drives(ORIGINAL1);
        let backup1_of_original1 = find_exist_folder_in_drives(BACKUP1_OF_ORIGINAL1);
        let backup2_of_original1 = find_exist_folder_in_drives(BACKUP2_OF_ORIGINAL1);
        let original2 = find_exist_folder_in_drives(ORIGINAL2);
        let backup_of_original2 = find_exist_folder_in_drives(BACKUP_OF_ORIGINAL2);

        Self {
            original1,
            backup1_of_original1,
            backup2_of_original1,
            original2,
            backup_of_original2,
            changing_fields: ChangingFields {
                files_changed: vec![],
                count_files_changed: 0,
                text_to_show: "".to_string(),
            },
        }
    }
}

impl MyApp {
    /// Mandatory method for the iced GUI library.
    /// It is the only place to change app state.
    fn update(&mut self, message: Message) {
        match message {
            Message::StartBackup => {
                self.start_all_backups_on_click();
            }
            Message::ExitProgram => {
                std::process::exit(0);
            }
        }
    }
    /// Mandatory method for the iced GUI library.
    /// It is the only place to draw the user interface after a change of the app state.
    fn view(&self) -> iced::Element<Message> {
        XScrollable::new(
            XScrollableAttr {
                width: iced::Length::Fixed(500.0),
                height: iced::Length::Fixed(600.0),
                ..Default::default()
            },
            iced::widget::container({
                let mut col = XColumn::new(XColumnAttr {
                    spacing: 5.0,
                    padding: 10.0.into(),
                    align_x: iced::alignment::Horizontal::Center,
                    width: iced::Length::Fill,
                });

                col.push(XText::attr_text(XTextAttr { size: 30.0 }, "Backup for Željko"));
                col.append(&mut vec![
                    XText::text("Simple backup program tailored for my friend Željko.").into(),
                    XText::text("Made with rust and iced GUI.").into(),
                    XText::text("https://github.com/bestia-dev/backup_for_zeljko").into(),
                    XText::text("© bestia.dev 2024 MIT license Open-source and free as a beer").into(),
                ]);

                col.push(XText::text("First backup:"));
                if let Some(backup1_of_original1) = &self.backup1_of_original1 {
                    if let Some(original1) = &self.original1 {
                        col.push(XText::text(format!("    {original1} ---> {backup1_of_original1}")));
                    } else {
                        col.push(XText::text(format!("    Folder {ORIGINAL1} does not exist!")));
                    }
                } else {
                    col.push(XText::text(format!("    Folder {BACKUP1_OF_ORIGINAL1} does not exist!")));
                }
                col.push(XText::text("Second backup:"));
                if let Some(backup2_of_original1) = &self.backup2_of_original1 {
                    if let Some(original1) = &self.original1 {
                        col.push(XText::text(format!("    {original1} ---> {backup2_of_original1}")));
                    } else {
                        col.push(XText::text(format!("    Folder {ORIGINAL1} does not exist!")));
                    }
                } else {
                    col.push(XText::text(format!("    Folder {BACKUP2_OF_ORIGINAL1} does not exist!")));
                }

                col.push(XText::text("Third backup:"));
                if let Some(backup_of_original2) = &self.backup_of_original2 {
                    if let Some(original2) = &self.original2 {
                        col.push(XText::text(format!("    {original2} ---> {backup_of_original2}")));
                    } else {
                        col.push(XText::text(format!("    Folder {ORIGINAL2} does not exist!")));
                    }
                } else {
                    col.push(XText::text(format!("    Folder {BACKUP_OF_ORIGINAL2} does not exist!")));
                }

                col.push({
                    // I can use blocks or block expressions to crate an isolated space for more complex code.
                    // https://doc.rust-lang.org/reference/expressions/block-expr.html
                    // It is like calling a function to isolate a gui "component".
                    // But this way the code is visible here and it is easier to follow.
                    // In their simple form, closures are very similar to block expressions.
                    let toolbar = iced::widget::row![
                        iced::widget::Button::new("Start backup").on_press(Message::StartBackup),
                        iced::widget::button("Exit program").on_press(Message::ExitProgram),
                    ]
                    .spacing(30)
                    .padding(iced::Padding::from(5));
                    // return
                    toolbar
                });

                col.push(XText::text(self.changing_fields.text_to_show.as_str()));

                // return the entire column converting it to its iced form
                col.to_iced()
            }),
        )
        .into()
    }

    /// Method that responds to the on click event.
    /// It starts the 3 different backups.
    fn start_all_backups_on_click(&mut self) {
        if let Some(original1) = self.original1.as_mut() {
            if let Some(backup1_of_original1) = self.backup1_of_original1.as_mut() {
                self.changing_fields.text_to_show.push_str("\nFirst backup\n");
                self.changing_fields.backup(original1, backup1_of_original1, "first");
            }
        }

        if let Some(original1) = self.original1.as_ref() {
            if let Some(backup2_of_original1) = self.backup2_of_original1.as_ref() {
                self.changing_fields.text_to_show.push_str("\nSecond backup\n");
                self.changing_fields.backup(original1, backup2_of_original1, "second");
            }
        }

        if let Some(original2) = self.original2.as_ref() {
            if let Some(backup_of_original2) = self.backup_of_original2.as_ref() {
                self.changing_fields.text_to_show.push_str("\nThird backup\n");
                self.changing_fields.backup(original2, backup_of_original2, "third");
            }
        }
        self.changing_fields
            .text_to_show
            .push_str(&format!("\nAll files changed for backup: {}\n", self.changing_fields.count_files_changed));
    }
}

impl ChangingFields {
    /// The backup() method uses robocopy. Additionally it saves the deleted and renamed files to a "deleted" folder.
    /// That way we can revert the changes if we find to be needed. After that the "deleted" folder can be removed forever.
    fn backup(&mut self, source: &str, destination: &str, backup_number: &str) {
        let output = Robocopy::command_robocopy_list_only(source, destination);
        self.files_changed = Robocopy::parse_robocopy_output(output);
        self.count_files_changed += self.files_changed.len();
        self.text_to_show.push_str(&self.files_changed.join("\n"));
        self.text_to_show.push('\n');

        // move the files instead of deleting them
        use chrono::{DateTime, Local};
        let current_local: DateTime<Local> = Local::now();
        let now_formatted = current_local.format("%Y-%m-%d_%H-%M-%S").to_string();
        // take the "e:\" part of destination to create the new folder
        let deleted_on_backup_folder = format!(r#"{}deleted_or_renamed_on_backup\{now_formatted}_{backup_number}_backup"#, &destination[..3]);
        // log::info!("{deleted_on_backup_folder}");
        for x in &self.files_changed {
            // only the destination folder and prepare to move them
            if x.starts_with(&destination) {
                let move_to = x.replace(&destination, &deleted_on_backup_folder);
                let parent_dir = unwrap!(std::path::Path::new(&move_to).parent());
                if !parent_dir.exists() {
                    unwrap!(std::fs::create_dir_all(&parent_dir));
                }
                unwrap!(std::fs::rename(x, move_to));
            }
        }
        Robocopy::command_robocopy_mir(source, destination);
    }
}
/// This struct is needed just to encapsulate robocopy methods.
/// Often it is easier to structure the code using struct and impl than using modules.
struct Robocopy {}
impl Robocopy {
    /// robocopy list only
    pub fn command_robocopy_list_only(source: &str, destination: &str) -> std::process::Output {
        use std::os::windows::process::CommandExt;
        let output = std::process::Command::new("robocopy")
            .args(&[source, destination, "/MIR", "/L", "/X", "/FP", "/NS", "/NC", "/NDL"])
            // specific windows flag to not open the terminal window
            .creation_flags(0x08000000)
            .output()
            .expect("failed to execute process");
        output
    }

    pub fn parse_robocopy_output(output: std::process::Output) -> Vec<String> {
        let mut vec_string: Vec<String> = vec![];
        // find the third line ------
        let mut count_del_lines = 0;
        // import the trait that has .lines()
        use std::io::BufRead;
        for x in output.stdout.lines() {
            let x = unwrap!(x);
            if x.starts_with("-----") {
                count_del_lines += 1;
            } else if count_del_lines == 3 && !x.is_empty() {
                vec_string.push(x.trim().to_string());
            }
        }
        vec_string
    }

    /// robocopy MIR
    pub fn command_robocopy_mir(source: &str, destination: &str) -> std::process::Output {
        use std::os::windows::process::CommandExt;
        let output = std::process::Command::new("robocopy")
            .args(&[source, destination, "/MIR"])
            // specific windows flag to not open the terminal window
            .creation_flags(0x08000000)
            .output()
            .expect("failed to execute process");
        output
    }
}

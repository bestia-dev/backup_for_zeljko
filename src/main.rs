// backup_for_zeljko/src/main.rs

// region: auto_md_to_doc_comments include README.md A //!

// endregion: auto_md_to_doc_comments include README.md A //!

// This app is intended just for Windows.
#![cfg(target_os = "windows")]
// In VSCode settings#![cfg(target_os = "windows")] I needed to set:
// "rust-analyzer.cargo.target": "x86_64-pc-windows-gnu"
// to avoid rust-analyzer to show me errors that are Linux specific,
// because I cross-compile this program from Linux to Windows.

// Do not open terminal when executing the program in windows
#![windows_subsystem = "windows"]

/// Use unwrap! macro to get the line and column of panics also in release mode.
// use unwrap::unwrap;
mod gui_helper;
use gui_helper::*;

static ORIGINAL_1: &str = r#"original_1"#;
static BACKUP_OF_ORIGINAL_1: &str = r#"backup_of_original_1"#;

fn main() {
    // scaffolding for catch panic and log to file
    tracing_init();

    let version: &'static str = env!("CARGO_PKG_VERSION");
    tracing::info!("Start app backup_for_zeljko v{}", version);

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
    tracing::error!("Panicked: ");
    if let Some(string) = payload.downcast_ref::<String>() {
        tracing::error!("{string}");
    } else if let Some(str) = payload.downcast_ref::<&'static str>() {
        tracing::error!("{str}")
    } else {
        tracing::error!("{payload:?}")
    }

    tracing::error!("Backtrace: {backtrace:#?}");
}

/// Initialize tracing to file logs/log.YYYY-MM-DD
///
/// The folder logs/ is in .gitignore and will not be committed.
pub fn tracing_init() {
    // tracing to file
    let file_appender = tracing_appender::rolling::daily("logs", "log");

    let offset = time::UtcOffset::current_local_offset().expect("should get local offset!");
    let timer = tracing_subscriber::fmt::time::OffsetTime::new(
        offset,
        time::macros::format_description!("[hour]:[minute]:[second].[subsecond digits:6]"),
    );

    // Filter out logs from dependency crates
    // A filter consists of one or more comma-separated directives
    // target[span{field=value}]=level
    // examples: wgpu_core=error
    // directives can be added with the RUST_LOG environment variable:
    // export RUST_LOG=backup_for_zeljko=trace
    // Unset the environment variable RUST_LOG
    // unset RUST_LOG
    let filter = tracing_subscriber::EnvFilter::from_default_env()
        .add_directive("backup_for_zeljko=debug".parse().unwrap_or_else(|e| panic!("{e}")))
        .add_directive("wgpu_core=error".parse().unwrap_or_else(|e| panic!("{e}")))
        .add_directive("iced_wgpu=error".parse().unwrap_or_else(|e| panic!("{e}")))
        .add_directive("iced_winit=error".parse().unwrap_or_else(|e| panic!("{e}")))
        .add_directive("wgpu_hal=error".parse().unwrap_or_else(|e| panic!("{e}")));

    tracing_subscriber::fmt()
        .with_file(true)
        .with_max_level(tracing::Level::DEBUG)
        .with_timer(timer)
        .with_line_number(true)
        .with_ansi(false)
        .with_writer(file_appender)
        .with_env_filter(filter)
        .init();
}

/// The original main() is cluttered with standard app scaffolding.
/// Here is only code that is app specific.
fn main_inner() -> iced::Result {
    let icon = iced::window::icon::from_file_data(include_bytes!("../images/flat_tyre_32x32.png"), None).expect("icon::from_file_data");
    iced::application("Backup for Željko", MyApp::update, MyApp::view)
        .window(iced::window::Settings {
            size: iced::Size::new(500.0, 600.0),
            icon: Some(icon),
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

/// GUI have different view pages
#[derive(Debug, Clone, Copy)]
enum ViewPage {
    Main,
    Original1NotFound,
    BackupOfOriginal1NotFound,
}

struct MyApp {
    original_1: Option<String>,
    backup_of_original_1: Option<String>,
    view_page: ViewPage,
}

impl Default for MyApp {
    /// Defaults are better to show intent then the "constructor" new() method.
    fn default() -> Self {
        /// Internal function, because it is called only once and only here.
        fn find_exist_folder_in_drives(path_wo_drive_letter: &str) -> Option<String> {
            let drives_letters = vec![
                "c", "d", "e", "f", "g", "h", "i", "j", "k", "l", "m", "n", "o", "p", "q", "r", "s", "t", "u", "v", "w", "x", "y", "z",
            ];
            for x in drives_letters.iter() {
                let path = format!(r#"{x}:\{path_wo_drive_letter}"#);
                if std::fs::exists(&path).expect("fs::exists") {
                    return Some(path);
                }
            }
            None
        }

        let mut view_page = ViewPage::Main;
        // External drives can have different letters d:, e:, f:,...
        // I need to check where is the foldername I expect. The folder names are fixed.
        let original_1 = find_exist_folder_in_drives(ORIGINAL_1);
        if original_1.is_none() {
            view_page = ViewPage::Original1NotFound;
        }
        let backup_of_original_1 = find_exist_folder_in_drives(BACKUP_OF_ORIGINAL_1);
        if backup_of_original_1.is_none() {
            view_page = ViewPage::BackupOfOriginal1NotFound;
        }
        Self {
            original_1,
            backup_of_original_1,
            view_page,
        }
    }
}

impl MyApp {
    /// Mandatory method for the iced GUI library.
    /// It is the only place to change app state.
    fn update(&mut self, message: Message) {
        match message {
            Message::StartBackup => {
                self.start_backup_on_click();
            }
            Message::ExitProgram => {
                std::process::exit(0);
            }
        }
    }
    /// Mandatory method for the iced GUI library.
    /// It is the only place to draw the user interface after a change of the app state.
    fn view(&self) -> iced::Element<Message> {
        XScrollable::new_scrollable(
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
                match self.view_page {
                    ViewPage::Main => self.view_page_main(&mut col),
                    ViewPage::Original1NotFound => self.view_page_original_1_not_found(&mut col),
                    ViewPage::BackupOfOriginal1NotFound => self.view_page_backup_of_original_1_not_found(&mut col),
                }

                // return the entire column converting it to its iced form
                col.into_iced()
            }),
        )
        .into()
    }

    fn view_page_main(&self, col: &mut XColumn<'_>) {
        col.push(XText::attr_text(
            XTextAttr {
                size: Some(30.0),
                ..Default::default()
            },
            "Backup for Željko",
        ));

        col.push(XText::text(String::new()));

        col.append(&mut vec![
            XText::text("Simple backup program tailored for my friend Željko.").into(),
        ]);

        col.push(XText::text(String::new()));

        col.push(XText::text("Backup:"));
        if let Some(backup_of_original_1) = &self.backup_of_original_1 {
            if let Some(original_1) = &self.original_1 {
                col.push(XText::text(format!("    {original_1} ---> {backup_of_original_1}")));
            } else {
                col.push(XText::text(format!("    Folder {ORIGINAL_1} does not exist!")));
            }
        } else {
            col.push(XText::text(format!("    Folder {BACKUP_OF_ORIGINAL_1} does not exist!")));
        }

        col.push(XText::text(String::new()));

        col.push(XText::text("Click on \"Start backup\" to open the console window."));
        col.push(XText::text("There will be run the robocopy command."));
        col.push(XText::text("All the changed files will be clearly listed."));
        col.push(XText::text("After that you can close the console window"));
        col.push(XText::text("and click on \"Exit program\""));

        col.push(XText::text(String::new()));

        col.push({
            // I can use blocks or block expressions to create an isolated space for more complex code.
            // https://doc.rust-lang.org/reference/expressions/block-expr.html
            // It is like calling a function to isolate a gui "component".
            // But this way the code is visible here and it is easier to follow.
            // In their simple form, closures are very similar to block expressions.
            iced::widget::row![
                iced::widget::Button::new("Start backup").on_press(Message::StartBackup),
                iced::widget::button("Exit program").on_press(Message::ExitProgram),
            ]
            .spacing(30)
            .padding(iced::Padding::from(5))
            // return toolbar
        });

        col.push(XText::text(String::new()));

        col.append(&mut vec![
            XText::text("https://github.com/bestia-dev/backup_for_zeljko").into(),
            XText::text("Made with rust and iced GUI.").into(),
            XText::text("© bestia.dev 2024 MIT license Open-source and free as a beer").into(),
        ]);
    }

    fn view_page_original_1_not_found(&self, col: &mut XColumn<'_>) {
        col.push(XText::attr_text(
            XTextAttr {
                size: Some(30.0),
                ..Default::default()
            },
            "Backup for Željko",
        ));

        col.push(XText::text(String::new()));

        col.push(XText::attr_text(
            XTextAttr {
                color: Some(iced::Color::parse("#FF0000").expect("Color::parse")),
                ..Default::default()
            },
            "Error: Folder original_1 not found on any drive!",
        ));

        col.push(XText::text(String::new()));

        col.push({
            iced::widget::row![iced::widget::button("Exit program").on_press(Message::ExitProgram),]
                .spacing(30)
                .padding(iced::Padding::from(5))
            // return toolbar
        });
    }

    fn view_page_backup_of_original_1_not_found(&self, col: &mut XColumn<'_>) {
        col.push(XText::attr_text(
            XTextAttr {
                size: Some(30.0),
                ..Default::default()
            },
            "Backup for Željko",
        ));

        col.push(XText::text(String::new()));

        col.push(XText::attr_text(
            XTextAttr {
                color: Some(iced::Color::parse("#FF0000").expect("Color::parse")),
                ..Default::default()
            },
            "Error: Folder backup_of_original_1 not found on any drive!",
        ));

        col.push(XText::text(String::new()));

        col.push({
            iced::widget::row![iced::widget::button("Exit program").on_press(Message::ExitProgram),]
                .spacing(30)
                .padding(iced::Padding::from(5))
            // return toolbar
        });
    }
    /// Method that responds to the on click event.
    fn start_backup_on_click(&mut self) {
        if let Some(original_1) = self.original_1.as_mut()
            && let Some(backup_of_original_1) = self.backup_of_original_1.as_mut()
        {
            Robocopy::command_robocopy_mir(original_1, backup_of_original_1);
        }
    }
}

/// This struct is needed just to encapsulate robocopy methods.
/// Often it is easier to structure the code using struct and impl than using modules.
/// robocopy d:\original_1 d:\backup_of_original_1 /MIR /FFT
struct Robocopy {}
impl Robocopy {
    /// robocopy MIR
    pub fn command_robocopy_mir(source: &str, destination: &str) {
        // /FFT Assumes FAT file times (two-second precision)
        // /NDL Specifies that directory names are not to be logged.
        let robocopy_cmd = format!("robocopy {source} {destination} /MIR /FFT /NDL");
        tracing::info!("{robocopy_cmd}");
        use std::os::windows::process::CommandExt;
        std::process::Command::new("cmd.exe")
            // CREATE_NEW_CONSOLE 0x00000010
            .creation_flags(0x00000010)
            // /K - Carries out the command specified by string but remains
            .arg("/K")
            .arg(&robocopy_cmd)
            .spawn()
            .expect("Command::new")
            .wait()
            .unwrap();
    }
}

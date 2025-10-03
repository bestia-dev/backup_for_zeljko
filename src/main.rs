// backup_for_zeljko/src/main.rs

// region: auto_md_to_doc_comments include README.md A //!
//! # backup_for_zeljko
//!
//! **Simple backup program tailored for my friend Željko. Made with rust and iced.**  
//! ***version: 2025.1003.856 date: 2025-10-03 author: [bestia.dev](https://bestia.dev) repository: [GitHub](https://github.com/bestia-dev/backup_for_zeljko)***
//!
//!  ![maintained](https://img.shields.io/badge/maintained-green)
//!  ![ready-for-use](https://img.shields.io/badge/ready_for_use-green)
//!  ![tutorial](https://img.shields.io/badge/tutorial-orange)
//!  ![iced](https://img.shields.io/badge/iced-orange)
//!  ![rust](https://img.shields.io/badge/rust-orange)
//!  ![gui](https://img.shields.io/badge/gui-orange)
//!
//!  ![License](https://img.shields.io/badge/license-MIT-blue.svg)
//!  ![backup_for_zeljko](https://bestia.dev/webpage_hit_counter/get_svg_image/2117022954.svg)
//!
//! Hashtags: #tutorial #iced #rust #gui  
//! My projects on GitHub are more like a tutorial than a finished product: [bestia-dev tutorials](https://github.com/bestia-dev/tutorials_rust_wasm).
//!
//! ## Storage
//!
//! My friend needs to store about 1,5TB of files, mostly photos.
//! He has 2 laptops with little storage on the internal SSD. We could never use the internal storage to store all of his files. We can use the internal SSD just for "temporary working copies" of some of the files. Eventually we must store the files to a persistent storage.
//!
//! 2TB should be enough to store all his files.  
//! He bought a 2TB super tiny and extremely fast Samsung T7 Shield external SSD. This will be the main persistent storage of his "original files".  
//! He has also a 2TB old external HDD as the backup drive.  
//!
//! ## Original location
//!
//! To avoid any confusion, files and folder must have one and only one "original location".  If anything happens, we know where to find our "original files". The fast external SSD is great for that. This will be the persistent storage of all his "original files".
//!
//! For some projects it is best to work directly with the "original files". For example when renaming photos and folders.
//!
//! ## Temporary working copy
//!
//! When working on a project with many small files, first copy the "original files" from the external SSD into the internal "temporary working copy" folder.  
//!
//! Work for some time on that files.  
//!
//! When a part of the project is finished, copy the files to their "original location". That can be done manually with great control. Do this often and with confidence. The "original" will survive, all the rest is just temporary.
//!
//! When the project is finished and all the files are copied into "original location" you do not needed the "temporary working files" any more. Delete these files from the "temporary working copy" to avoid any confusion later. The location of the "originals files" must be always in the same place: the external SSD.
//!
//! ## Backup
//!
//! In the computer world anything can stop working in a second without any warning signs. I witnessed people loosing all their files because a hard disk died. I saw them cry.
//!
//! We must always have a backup of the files. It is tedious and it looks superfluous until the disaster strikes.
//!
//! I will use `robocopy` to make the backup or mirror from the external SSD to the old external HDD.
//!
//! Never change manually the files on the backup HDD. Only use the robocopy command.
//!
//! ## GUI for windows
//!
//! My friend is not a computer guy, so I decided that a CLI program in a terminal is not for him. He is comfortable to use GUI programs in Windows.
//!
//! I will use the crate `iced` to create a simple GUI program for Windows. It is "retained mode GUI".
//!
//! First the program will check what disks are connected. The names of the folders are fixed so I can recognize them easily. The letters of the external disks can get mounted differently, therefore there is an exploration phase.
//!
//! 1. x:\original_1
//! 2. y:\backup_of_original_1
//!
//! I will use `robocopy` to make a "mirror backup". Sounds easy.
//!
//! ## Cross compile to windows
//!
//! On my machine I have Windows11 with WSL/Debian. I will cross compile to Windows, copy the exe file with `scp` and run it on Windows.  
//!
//! I use `cargo-auto` for automation of the build process and to commit to GitHub. Just run `cargo auto` and follow the instructions. To work with GitHub it will need the Personal Access Token from <https://github.com/settings/tokens>.  
//!
//! Copy the exe file from the container 'crustde' to win folder. Run in windows git-bash:
//!
//! ```bash
//! mkdir -p ~/git-bash/rustprojects/backup_for_zeljko
//! cd ~/git-bash/rustprojects/backup_for_zeljko
//! scp rustdevuser@crustde:/home/rustdevuser/rustprojects/backup_for_zeljko/target/x86_64-pc-windows-gnu/release/backup_for_zeljko.exe /c/Users/Luciano/git-bash/rustprojects/backup_for_zeljko/
//!
//! # then run in git-bash
//! ./backup_for_zeljko.exe
//! ```
//!
//! ## Robocopy
//!
//! Robocopy stands for "Robust copy" in Windows. It is good.  
//!
//!
//! ```bash
//! robocopy options
//! /X :: report all eXtra files, not just those selected.
//! /FP :: include Full Pathname of files in the output.
//! /NS :: No Size - don't log file sizes.
//! /NC :: No Class - don't log file classes.
//! /NDL :: No Directory List - don't log directory names.
//!
//! robocopy d:\original_1 d:\backup_of_original_1 /MIR /X /FP /NS /NC /NDL
//!
//! ```
//!
//! ## Open-source and free as a beer
//!
//! My open-source projects are free as a beer (MIT license).  
//! I just love programming.  
//! But I need also to drink. If you find my projects and tutorials helpful, please buy me a beer by donating to my [PayPal](https://paypal.me/LucianoBestia).  
//! You know the price of a beer in your local bar ;-)  
//! So I can drink a free beer for your health :-)  
//! [Na zdravje!](https://translate.google.com/?hl=en&sl=sl&tl=en&text=Na%20zdravje&op=translate) [Alla salute!](https://dictionary.cambridge.org/dictionary/italian-english/alla-salute) [Prost!](https://dictionary.cambridge.org/dictionary/german-english/prost) [Nazdravlje!](https://matadornetwork.com/nights/how-to-say-cheers-in-50-languages/) 🍻
//!
//! [//bestia.dev](https://bestia.dev)  
//! [//github.com/bestia-dev](https://github.com/bestia-dev)  
//! [//bestiadev.substack.com](https://bestiadev.substack.com)  
//! [//youtube.com/@bestia-dev-tutorials](https://youtube.com/@bestia-dev-tutorials)  
//!
// endregion: auto_md_to_doc_comments include README.md A //!

// This whole app is intended just for Windows.
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
    fn view(&self) -> iced::Element<'_, Message> {
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

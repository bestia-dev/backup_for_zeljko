// automation_tasks_rs for backup_for_zeljko

// region: library and modules with basic automation tasks

mod build_cli_bin_mod;
mod build_cli_bin_win_mod;
mod build_lib_mod;
mod cargo_auto_github_api_mod;
mod encrypt_decrypt_with_ssh_key_mod;
mod generic_functions_mod;
mod tasks_mod;


pub use cargo_auto_lib as cl;

use crate::cargo_auto_github_api_mod as cgl;
use crate::encrypt_decrypt_with_ssh_key_mod as ende;
use crate::generic_functions_mod as gn;
use crate::tasks_mod as ts;

pub use cl::{BLUE, GREEN, RED, RESET, YELLOW};

// traits must be in scope (Rust strangeness)
use cl::CargoTomlPublicApiMethods;

// region: library with basic automation tasks

///main returns ExitCode
fn main() -> std::process::ExitCode {
    match main_returns_anyhow_result() {
        Err(err) => {
            eprintln!("{}", err);
            // eprintln!("Exit program with failure exit code 1");
            std::process::ExitCode::FAILURE
        }
        Ok(()) => std::process::ExitCode::SUCCESS,
    }
}

/// main() returns anyhow::Result
fn main_returns_anyhow_result() -> anyhow::Result<()> {
    gn::tracing_init()?;
    cl::exit_if_not_run_in_rust_project_root_directory();
    ende::github_api_token_with_oauth2_mod::github_api_config_initialize()?;
    ende::crates_io_api_token_mod::crates_io_config_initialize()?;
    // get CLI arguments
    let mut args = std::env::args();
    // the zero argument is the name of the program
    let _arg_0 = args.next();
    match_arguments_and_call_tasks(args)?;
    Ok(())
}

// region: match, help and completion

/// match arguments and call tasks functions
fn match_arguments_and_call_tasks(mut args: std::env::Args)->anyhow::Result<()> {
    // the first argument is the user defined task: (no argument for help), build, release,...
    let arg_1 = args.next();
    match arg_1 {
        None => print_help()?,
        Some(task) => {
            if &task == "completion" {
                completion()?;
            } else {
                println!("  {YELLOW}Running automation task: {task}{RESET}");
                if &task == "build_win" {
                    task_build_win()?;
                } else if &task == "win_release" {
                    task_win_release()?;
                } else if &task == "doc" {
                    task_doc()?;
                } else if &task == "test" {
                    task_test()?;
                } else if &task == "commit_and_push" {
                    let arg_2 = args.next();
                    task_commit_and_push(arg_2)?;
                } else if &task == "github_new_release" {
                    task_github_new_release()?;
                } else {
                    eprintln!("{RED}Error: Task {task} is unknown.{RESET}");
                    print_help()?;
                }
            }
        }
    }
    Ok(())
}

/// write a comprehensible help for user defined tasks
fn print_help() ->anyhow::Result<()>{
    println!(
        r#"
  {YELLOW}Welcome to cargo-auto !{RESET}
  {YELLOW}This program automates your custom tasks when developing a Rust project.{RESET}

  {YELLOW}User defined tasks in automation_tasks_rs:{RESET}
{GREEN}cargo auto build_win{RESET} - {YELLOW}builds the crate in debug mode, fmt, increment version{RESET}
{GREEN}cargo auto win_release{RESET} - {YELLOW}builds the crate in release mode (cross compile to win), fmt, increment version{RESET}
{GREEN}cargo auto doc{RESET} - {YELLOW}builds the docs, copy to docs directory{RESET}
{GREEN}cargo auto test{RESET} - {YELLOW}runs all the tests{RESET}
{GREEN}cargo auto commit_and_push "message"{RESET} - {YELLOW}commits with message and push with mandatory message{RESET}
  {YELLOW}It is preferred to use SSH for git push to GitHub.{RESET}
  {YELLOW}<https://github.com/CRUSTDE-ContainerizedRustDevEnv/crustde_cnt_img_pod/blob/main/ssh_easy.md>{YELLOW}
  {YELLOW}On the very first commit, this task will initialize a new local git repository and create a remote GitHub repo.{RESET}
  {YELLOW}For the GitHub API the task needs the Access secret token from OAuth2 device workflow.{RESET}
  {YELLOW}The secret token will be stored in a file encrypted with your SSH private key.{RESET}
  {YELLOW}You can type the passphrase of the private key for every usee. This is pretty secure.{RESET}
  {YELLOW}Somewhat less secure (but more comfortable) way is to store the private key in ssh-agent.{RESET}
{GREEN}cargo auto github_new_release{RESET} - {YELLOW}creates new release on GitHub{RESET}
  {YELLOW}For the GitHub API the task needs the Access secret token from OAuth2 device workflow.{RESET}
  {YELLOW}The secret token will be stored in a file encrypted with your SSH private key.{RESET}
  {YELLOW}You can type the passphrase of the private key for every usee. This is pretty secure.{RESET}
  {YELLOW}Somewhat less secure (but more comfortable) way is to store the private key in ssh-agent.{RESET}
{GREEN}cargo auto update_automation_tasks_rs{RESET} - {YELLOW}updates the files in automation_tasks_rs{RESET}
  {YELLOW}Some files are fixed and the update is straight forward, other files need manual diff.{RESET}

  {YELLOW}© 2025 bestia.dev  MIT License github.com/automation-tasks-rs/cargo-auto{RESET}
"#
    );
    print_examples_cmd();
    Ok(())
}

/// all example commands in one place
fn print_examples_cmd() {
    /*
        println!(
            r#"
    {YELLOW}run examples:{RESET}
    {GREEN}cargo run --example plantuml1{RESET}
    "#
        );
    */
}

/// Sub-command for bash auto-completion of `cargo auto` using the crate `dev_bestia_cargo_completion`.
fn completion() ->anyhow::Result<()>{
    let args: Vec<String> = std::env::args().collect();
    let word_being_completed = args[2].as_str();
    let last_word = args[3].as_str();

    if last_word == "cargo-auto" || last_word == "auto" {
        let sub_commands = vec![
            "build_win",
            "win_release",
            "doc",
            "test",
            "commit_and_push",
            "github_new_release",
            "update_automation_tasks_rs"
        ];
        cl::completion_return_one_or_more_sub_commands(sub_commands, word_being_completed);
    }
    /*
    // the second level if needed
    else if last_word == "new" {
        let sub_commands = vec!["x"];
       cl::completion_return_one_or_more_sub_commands(sub_commands, word_being_completed);
    }
    */
    Ok(())
}

// endregion: match, help and completion

// region: tasks

/// cargo build_win
fn task_build_win() -> anyhow::Result<()>{
    let cargo_toml = crate::build_cli_bin_win_mod::task_build()?;

    println!(
        r#"
  {YELLOW}After `cargo auto build_win`, run the compiled binary, examples and/or tests{RESET}
  {YELLOW}if {package_name} ok then{RESET}
{GREEN}cargo auto win_release{RESET}
"#,
        package_name = cargo_toml.package_name(),
    );
    print_examples_cmd();
    Ok(())
}

/// cargo build --release --target x86_64-pc-windows-gnu
/// TODO: try cross compile to windows, because Linux has problems with file datetime on external disk
fn task_win_release() -> anyhow::Result<()>{
    let cargo_toml = crate::build_cli_bin_win_mod::task_release()?;

    println!(
        r#"
    {YELLOW}After `cargo auto win_release`, run the compiled binary, examples and/or tests{RESET}

    {YELLOW}In Windows git-bash, copy the exe file from the Crustde container to Windows.{RESET}
{GREEN}
mkdir -p ~/git-bash/rustprojects/{package_name} 
cd ~/git-bash/rustprojects/{package_name}
scp rustdevuser@crustde:/home/rustdevuser/rustprojects/{package_name}/target/x86_64-pc-windows-gnu/release/{package_name}.exe /c/Users/Luciano/git-bash/rustprojects/{package_name}/{RESET}
    {YELLOW}Run the exe in Windows git-bash.{RESET}
{GREEN}
./{package_name}.exe{RESET}

    {YELLOW}if ok then{RESET}
{GREEN}cargo auto doc{RESET}
"#,
        package_name = cargo_toml.package_name(),
    );
    print_examples_cmd();
    Ok(())
}

/// cargo doc, then copies to /docs/ folder, because this is a GitHub standard folder
fn task_doc() ->anyhow::Result<()>{
    ts::task_doc()?;
    // message to help user with next move
    println!(
        r#"
  {YELLOW}If ok then run the tests in code and the documentation code examples.{RESET}
{GREEN}cargo auto test{RESET}
"#
    );
    Ok(())
}

/// cargo test
fn task_test()-> anyhow::Result<()> {
    println!(r"  {RED}backup_for_zeljko will be always cross build from Linux for Windows.
  There is not possible to test it on Linux.
  You have to copy the executable to Windows and test it there.{RESET}");

    //cl::run_shell_command_static("cargo test")?;

    println!(
        r#"
  {YELLOW}After `cargo auto test`. If ok then {RESET}
  {YELLOW}(commit message is mandatory){RESET}
{GREEN}cargo auto commit_and_push "message"{RESET}
"#
    );
    Ok(())
}

/// commit and push
fn task_commit_and_push(arg_2: Option<String>)->anyhow::Result<()> {
    ts::task_commit_and_push(arg_2)?;
    println!(
        r#"
  {YELLOW}After `cargo auto commit_and_push "message"`{RESET}
  {YELLOW}Now, write the content of the release in the RELEASES.md in the `## Unreleased` section, then{RESET}
  {YELLOW}Next, create the GitHub Release.{RESET}
{GREEN}cargo auto github_new_release{RESET}
"#
    );
    Ok(())
}

/// create a new release on github and uploads binary executables
fn task_github_new_release()->anyhow::Result<()> {
    ts::task_github_new_release()?;
    println!(
        r#"  
  {YELLOW}No more automation tasks. {RESET}
"#
    );
    Ok(())
}
// endregion: tasks

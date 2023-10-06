use crate::build_vars::is_using_sidecar;
use std::process::Command as StdCommand;
use tauri::api::process::Command;

pub fn ffmpeg_command() -> StdCommand {
    if is_using_sidecar() {
        let sidecar: Command =
            Command::new_sidecar("ffmpeg").expect("Failed to create ffmpeg sidecar");
        let command: StdCommand = StdCommand::from(sidecar);
        return command;
    }
    println!("NOT USING SIDECAR");

    StdCommand::new("ffmpeg")
}

pub fn ffprobe_command() -> StdCommand {
    if is_using_sidecar() {
        let sidecar: Command =
            Command::new_sidecar("ffprobe").expect("Failed to create ffprobe sidecar");
        let command: StdCommand = StdCommand::from(sidecar);
        return command;
    }
    println!("NOT USING SIDECAR");

    StdCommand::new("ffprobe")
}

#[cfg(target_os = "windows")]
pub fn add_raw_arg(mut cmd: StdCommand, arg: String) -> StdCommand {
    use std::os::windows::process::CommandExt;

    cmd.raw_arg(arg);
    cmd
}

#[cfg(target_os = "linux")]
pub fn add_raw_arg(mut cmd: StdCommand, arg: String) -> StdCommand {
    cmd.arg(arg);
    cmd
}

use tauri::api::process::Command;
use std::process::Command as StdCommand;
use crate::build_vars::is_using_sidecar;

pub fn ffmpeg_command() -> StdCommand {
  if is_using_sidecar() {
    let sidecar: Command = Command::new_sidecar("ffmpeg").expect("Failed to create ffmpeg sidecar");
    let command: StdCommand = StdCommand::from(sidecar);
    return command;
  }
  println!("NOT USING SIDECAR");
  let command = StdCommand::new("ffmpeg");
  return command;
}

pub fn ffprobe_command() -> StdCommand {
  if is_using_sidecar() {
    let sidecar: Command = Command::new_sidecar("ffprobe").expect("Failed to create ffprobe sidecar");
    let command: StdCommand = StdCommand::from(sidecar);
    return command;
  }
  println!("NOT USING SIDECAR");
  let command = StdCommand::new("ffprobe");
  return command;
}
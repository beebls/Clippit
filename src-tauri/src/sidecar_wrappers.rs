use tauri::api::process::Command;
use std::process::Command as StdCommand;

pub fn ffmpeg_command() -> StdCommand {
  let sidecar: Command = Command::new_sidecar("ffmpeg").expect("Failed to create ffmpeg sidecar");
  let command: StdCommand = StdCommand::from(sidecar);
  return command;
}

pub fn ffprobe_command() -> StdCommand {
  let sidecar: Command = Command::new_sidecar("ffprobe").expect("Failed to create ffprobe sidecar");
  let command: StdCommand = StdCommand::from(sidecar);
  return command;
}
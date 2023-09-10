// Prevents additional console window on Windows in release, DO NOT REMOVE!!
#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]

use std::os::windows::process::CommandExt;
use std::process::Command;
use std::fs;
use directories::BaseDirs;
use std::path::{Path, PathBuf};
use std::collections::hash_map::DefaultHasher;
use std::hash::{Hash, Hasher};


fn main() {
  tauri::Builder::default()
    .invoke_handler(tauri::generate_handler![get_num_audio_tracks, start_project, export_project])
    .run(tauri::generate_context!())
    .expect("error while running tauri application");
}

#[tauri::command]
async fn start_project(file_name: String) -> (i32, String) {
  let temp_hash: String = create_temp_dir(file_name.to_owned()).await;
  let num_audio_tracks: i32 = get_num_audio_tracks(file_name.to_owned()).await;


  let temp_root = get_temp_root().await;
  if temp_root.is_none() {
    return (1, String::from(""));
  }
  let full_temp_path: PathBuf = temp_root.unwrap().join(&temp_hash);

  extract_video_from_file(file_name.to_owned(), &full_temp_path).await;

  let mut acc = 0;
  while acc < num_audio_tracks {
  println!("Creating audio file {}", acc);
    split_out_single_audio_track(acc, file_name.to_owned(), &full_temp_path).await;
    acc = acc + 1;
  }
  println!("Created temp dir with hash {}, and the audio file has {} tracks", temp_hash, num_audio_tracks);
  println!("output dir {}", full_temp_path.to_string_lossy());
  return (num_audio_tracks, temp_hash)
}

#[tauri::command]
async fn get_num_audio_tracks(file_name: String) -> i32 {
  // TODO: THIS HAS NO FUCKING ERROR CHECKING
  let value: std::process::Output = Command::new("ffprobe").args(["-v", "error", "-select_streams", "a", "-show_entries", "stream=index", "-of", "csv=p=0", &file_name]).output().expect("Failed to execute process");
  let out: std::borrow::Cow<'_, str> = String::from_utf8_lossy(&value.stdout);
  let err: std::borrow::Cow<'_, str> = String::from_utf8_lossy(&value.stderr);
  let num_lines: i32 = out.split("\n").collect::<Vec<&str>>().len() as i32 - 1;
  println!("{} {}", num_lines, err);
  return num_lines;
}

async fn extract_video_from_file(input_file: String, output_dir: &PathBuf) {
  let value: std::process::Output = Command::new("ffmpeg").args(["-i", &input_file, "-map", "0:v:0", "-c", "copy", &output_dir.join("video.mp4").to_string_lossy()]).output().expect("Failed to execute process");
  let out = String::from_utf8_lossy(&value.stdout);
  let err = String::from_utf8_lossy(&value.stderr);

  println!("{} err {}", out, err);
}

async fn split_out_single_audio_track(track_num: i32, input_file: String, output_dir: &PathBuf) {
  let value: std::process::Output = Command::new("ffmpeg").args(["-i", &input_file, "-map", &format!("0:a:{}", track_num), &output_dir.join(format!("track_{}.mp3", track_num)).to_string_lossy()]).output().expect("Failed to execute process");
  let out = String::from_utf8_lossy(&value.stdout);
  let err = String::from_utf8_lossy(&value.stderr);

  println!("{} err {}", out, err);
}

async fn get_temp_root() -> Option<PathBuf> {
  if let Some(base_dirs) = BaseDirs::new() {
    let cache_root: &Path = base_dirs.cache_dir();
    let temp_root: PathBuf = cache_root.join("com.tauri.dev").join("temp");
    println!("{}", &temp_root.to_string_lossy());
    if !temp_root.exists() {
      println!("Temp Dir does not exist, creating");
      fs::create_dir_all(&temp_root).expect("Failed to create dir");
    }
    return Some(temp_root);
  }
  return None;
}

#[tauri::command]
async fn create_temp_dir(file_name: String) -> String {
  let temp_root = get_temp_root().await;
  if temp_root.is_none() {
    return String::from("ERROR: Unable To Find Temp Dir");
  }

  // Hash the file
  // TODO: POTENTIALLY WANT TO HAVE IT BE RANDOM INSTEAD OF HASH BECAUSE TWO FILES COULD HAVE THE SAME NAME
  let mut hasher = DefaultHasher::new();
  file_name.hash(&mut hasher);
  let hash = hasher.finish().to_string();

  // Create temp dir from hash
  println!("Creating temp files dir with hash {}", &hash);
  let _ = fs::create_dir_all(temp_root.unwrap().join(&hash));
  return hash;
}

// new_height will be set to 0 if it shouldn't be scaled
#[tauri::command]
async fn export_project(project_hash: String, start_time: f32, end_time: f32, audio_volumes: Vec<f32>, output_file: String, new_height: i32, new_megabytes: f32, encoder_type: String) {
  create_output_dir(project_hash.to_owned()).await;

  for (index, element) in audio_volumes.iter().enumerate() {
    format_audio_track(index as i32, project_hash.to_owned(), element.to_owned(), start_time, end_time).await;
  }

  let formatted_height: Option<i32>;
  if new_height == 0 {
    formatted_height = None;
  } else {
    formatted_height = Some(new_height);
  }

  let formatted_megabytes: Option<f32>;
  if new_megabytes == 0.0 {
    formatted_megabytes = None;
  } else {
    formatted_megabytes = Some(new_megabytes);
  }

  let mut formatted_encoder: String = "x264".to_owned();

  let x264_values = ["h264".to_owned(), "x264".to_owned(), "libx264".to_owned()];
  let x265_values = ["h265".to_owned(), "x265".to_owned(), "libx265".to_owned()];
  if x265_values.contains(&encoder_type) {
    formatted_encoder = "x265".to_owned();
  } else if x264_values.contains(&encoder_type) {
    formatted_encoder = "x264".to_owned();
  }


  if formatted_megabytes.is_some() {
    let temp_root = get_temp_root().await;
    if temp_root.is_none() {
      // return String::from("ERROR: Unable To Find Temp Dir");
      return;
    }
    let full_temp_path: PathBuf = temp_root.unwrap().join(&project_hash);
    let temp_output_path = full_temp_path.join("output");
    let downscaled_video_path = temp_output_path.join("downscaled.mp4");

    merge_audios_into_video_and_downscale(project_hash.to_owned(), audio_volumes.len() as i32, start_time, end_time, downscaled_video_path.to_string_lossy().to_string(), formatted_height).await;

    encode_video_to_specific_filesize(project_hash, downscaled_video_path.to_string_lossy().to_string(), output_file, formatted_megabytes.unwrap(), formatted_encoder).await;
  } else {
    merge_audios_into_video_and_downscale(project_hash, audio_volumes.len() as i32, start_time, end_time, output_file, formatted_height).await;
  }

}

async fn create_output_dir(project_hash: String) {
  let temp_root = get_temp_root().await;
  if temp_root.is_none() {
    // return String::from("ERROR: Unable To Find Temp Dir");
    return;
  }
  let full_temp_path: PathBuf = temp_root.unwrap().join(&project_hash);
  let _ = fs::create_dir_all(full_temp_path.join("output"));

}

async fn format_audio_track(track_num: i32, project_hash: String, volume: f32, start_time: f32, end_time: f32){
  let temp_root = get_temp_root().await;
  if temp_root.is_none() {
    // return String::from("ERROR: Unable To Find Temp Dir");
    return;
  }
  let track_file_name = format!("track_{}.mp3", track_num);

  let full_temp_path: PathBuf = temp_root.unwrap().join(&project_hash);
  let input_path = full_temp_path.join(&track_file_name);
  let output_path = full_temp_path.join("output").join(&track_file_name);
  let _: std::process::Output = Command::new("ffmpeg").args(["-ss", &start_time.to_string(), "-to", &end_time.to_string(), "-i", &input_path.to_string_lossy(), "-filter:a", &format!("volume={}", volume), &output_path.to_string_lossy()]).output().expect("Failed to execute process");
  // let out = String::from_utf8_lossy(&value.stdout);
  // let err = String::from_utf8_lossy(&value.stderr);

  // println!("{} err {}", out, err);
}

async fn merge_audios_into_video_and_downscale(project_hash: String, num_audio_files: i32, start_time: f32, end_time: f32, output_path: String, new_height: Option<i32>) {
  let temp_root = get_temp_root().await;
  if temp_root.is_none() {
    // return String::from("ERROR: Unable To Find Temp Dir");
    return;
  }
  let full_temp_path: PathBuf = temp_root.unwrap().join(&project_hash);
  let temp_output_path = full_temp_path.join("output");

  let video_path = full_temp_path.join("video.mp4");

  let mut audio_mux_string = String::from("");

  let mut command: Command = Command::new("ffmpeg");

  command.arg("-ss");
  command.arg(start_time.to_string());
  command.arg("-to");
  command.arg(end_time.to_string());
  command.arg("-i");
  command.arg(video_path.to_string_lossy().to_string());

  let mut acc: i32 = 0;
  while acc < num_audio_files {
    audio_mux_string.push_str(&format!("[{}:a]", acc + 1));
    let file_path = temp_output_path.join(format!("track_{}.mp3", acc));

    let file_str = file_path.to_string_lossy().to_string();
    command.arg("-i");
    command.arg(file_str);
    acc = acc + 1;
  }
  audio_mux_string.push_str(&format!("amix=inputs={}[a]", num_audio_files));
  if new_height.is_some() {
    command.arg("-vf");
    command.raw_arg(format!("scale=\"trunc(oh*a/2)*2:{}\"", new_height.unwrap()));
  }
  command.arg("-filter_complex");
  command.arg(audio_mux_string);
  command.arg("-map");
  command.arg("0");
  command.arg("-map");
  command.arg("[a]");
  command.arg(output_path);

  let value = command.output().expect("Failed to execute process");
  let out = String::from_utf8_lossy(&value.stdout);
  let err = String::from_utf8_lossy(&value.stderr);

  println!("{} err {}", out, err);
}

async fn encode_video_to_specific_filesize(project_hash: String, input_file: String, output_file: String, target_mebibytes: f32, encoder_type: String) {
  let temp_root = get_temp_root().await;
  if temp_root.is_none() {
    // return String::from("ERROR: Unable To Find Temp Dir");
    return;
  }
  let full_temp_path: PathBuf = temp_root.unwrap().join(&project_hash);
  let temp_output_path = full_temp_path.join("output");

  let duration_cmd: std::process::Output = Command::new("ffprobe").args(["-v", "error", "-show_entries", "format=duration", "-of", "csv=p=0", &input_file]).output().expect("Failed to execute process");
  let duration_str: String = String::from_utf8_lossy(&duration_cmd.stdout).to_string();
  let duration: f32 = duration_str.trim().parse::<f32>().unwrap();
  // let err: std::borrow::Cow<'_, str> = String::from_utf8_lossy(&value.stderr);
  // let num_lines: i32 = out.split("\n").collect::<Vec<&str>>().len() as i32 - 1;
  println!("Duration {}", duration);

  let audio_rate_cmd: std::process::Output = Command::new("ffprobe").args(["-v", "error", "-select_streams", "a:0", "-show_entries", "stream=bit_rate", "-of", "csv=p=0", &input_file]).output().expect("Failed to execute process");
  let audio_rate_str: String = String::from_utf8_lossy(&audio_rate_cmd.stdout).to_string();
  // This command prints out bits PER SECOND
  // we need to convert this to MEBIBITS per second
  let audio_rate_mebibits: f32 = audio_rate_str.trim().parse::<f32>().unwrap() / 1048576.0;

  if target_mebibytes < audio_rate_mebibits / 8.0 * duration {
    // ERROR: IDK
    println!("test {} {}", audio_rate_mebibits, audio_rate_mebibits / 8.0 * duration);
    return;
  }

  // this is to ensure you dont go above the original;
  const EXTRA_PADDING_MEBIBYTES: f32 = 1.0;

  let video_rate_mebibytes = (target_mebibytes - EXTRA_PADDING_MEBIBYTES - (audio_rate_mebibits / 8.0 * duration)) / duration;

  let mut pass_one = Command::new("ffmpeg");
  pass_one.current_dir(&temp_output_path);
  pass_one.arg("-y");
  pass_one.arg("-i");
  pass_one.arg(&input_file);
  pass_one.arg("-c:v");
  if encoder_type == "x265" {
    pass_one.arg("libx265");
  } else if encoder_type == "av1" {
    pass_one.arg("libaom-av1");
  } else {
    pass_one.arg("libx264");
  }
  pass_one.arg("-b:v");
  pass_one.arg(format!("{}k", video_rate_mebibytes * 8192.0));
  if encoder_type == "x265" {
    pass_one.arg("-x265-params");
    pass_one.arg("pass=1");
  } else if encoder_type == "av1" {
    // 
  } else {
    pass_one.arg("-pass");
    pass_one.arg("1");
  }
  pass_one.arg("-an");
  pass_one.arg("-f");
  pass_one.arg("mp4");
  if cfg!(windows) {
    pass_one.arg("NUL");
  } else if cfg!(unix) {
    pass_one.arg("/dev/null");
  }
  let pass_one_output = pass_one.output().expect("Failed to execute the command");

  let out = String::from_utf8_lossy(&pass_one_output.stdout);
  let err = String::from_utf8_lossy(&pass_one_output.stderr);

  println!("{} {}", out, err);
  println!("PASS ONE DONE");

  let mut pass_two = Command::new("ffmpeg");
  pass_two.current_dir(temp_output_path);
  pass_two.arg("-i");
  pass_two.arg(input_file);
  pass_two.arg("-c:v");
  if encoder_type == "x265" {
    pass_two.arg("libx265");
  } else if encoder_type == "av1" {
    pass_two.arg("libaom-av1");
  } else {
    pass_two.arg("libx264");
  }
  pass_two.arg("-b:v");
  pass_two.arg(format!("{}k", video_rate_mebibytes * 8192.0));
  if encoder_type == "x265" {
    pass_two.arg("-x265-params");
    pass_two.arg("pass=2");
  } else if encoder_type == "av1" {
    // 
  } else {
    pass_two.arg("-pass");
    pass_two.arg("2");
  }
  pass_two.arg("-c:a");
  pass_two.arg("aac");
  pass_two.arg("-b:a");
  pass_two.arg(format!("{}k", audio_rate_mebibits * 1024.0));
  pass_two.arg(output_file);

  let pass_two_output = pass_two.output().expect("Failed to execute the command");

  let out = String::from_utf8_lossy(&pass_two_output.stdout);
  let err = String::from_utf8_lossy(&pass_two_output.stderr);

  println!("{} {}", out, err);
  println!("PASS TWO DONE");
}
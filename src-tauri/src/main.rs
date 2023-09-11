// Prevents additional console window on Windows in release, DO NOT REMOVE!!
#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]

mod temp_dirs;
mod ffmpeg;
mod sidecar_wrappers;

use std::path::PathBuf;

use temp_dirs::{get_project_temp_dir,create_temp_dir,get_output_dir};
use ffmpeg::{extract_video_from_file,split_out_single_audio_track,get_num_audio_tracks,format_audio_track,merge_audios_into_video_and_downscale,encode_video_to_specific_filesize};

fn main() {
  tauri::Builder::default()
    .invoke_handler(tauri::generate_handler![start_project, export_project])
    .run(tauri::generate_context!())
    .expect("error while running tauri application");
}

#[tauri::command]
async fn start_project(file_name: String) -> Result<(i32, String), String> {
  let temp_hash: String = create_temp_dir(file_name.to_owned()).await;

  let num_tracks_res = get_num_audio_tracks(file_name.to_owned()).await;
  if num_tracks_res.is_err() {
    return Err("Cannot Get Number of Audio Tracks".to_owned());
  }
  let num_audio_tracks = num_tracks_res.unwrap();

  let temp_path_option: Option<PathBuf> = get_project_temp_dir(temp_hash.to_owned()).await;
  if temp_path_option.is_none() {
    return Err("Cannot Get Temp Dir".to_owned());
  }
  let full_temp_path = temp_path_option.unwrap();

  match extract_video_from_file(file_name.to_owned(), &full_temp_path).await {
    Ok(_) => {},
    Err(error) => return Err(format!("Error Extracting Video: {}", error))
  }

  let mut acc: i32 = 0;
  while acc < num_audio_tracks {
    println!("Creating audio file {}", acc);
    match split_out_single_audio_track(acc, file_name.to_owned(), &full_temp_path).await {
      Ok(_) => acc = acc + 1,
      Err(error) => return Err(format!("Error Extracting Audio Track {}: {}", acc, error)),
    };
  
  }
  println!("Created temp dir with hash {}, and the audio file has {} tracks", temp_hash, num_audio_tracks);
  println!("output dir {}", full_temp_path.to_string_lossy());
  return Ok((num_audio_tracks, temp_hash));
}

// new_height will be set to 0 if it shouldn't be scaled
#[tauri::command]
async fn export_project(project_hash: String, start_time: f32, end_time: f32, audio_volumes: Vec<f32>, output_file: String, new_height: i32, new_megabytes: f32, encoder_type: String) -> Result<String, String> {
  let output_dir_res = get_output_dir(project_hash.to_owned()).await;
  if output_dir_res.is_none() {
    return Err("Cannot create temp output dir".to_owned());
  }
  let output_dir = output_dir_res.unwrap();

  for (index, element) in audio_volumes.iter().enumerate() {
    let res = format_audio_track(index as i32, project_hash.to_owned(), element.to_owned(), start_time, end_time).await;
    if res.is_err() {
      return Err(format!("Error formatting audio track {}: {}", index, res.unwrap_err()));
    }
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
    let downscaled_video_path: PathBuf = output_dir.join("downscaled.mp4");

    let downscale_res: Result<(), String> = merge_audios_into_video_and_downscale(project_hash.to_owned(), audio_volumes.len() as i32, start_time, end_time, downscaled_video_path.to_string_lossy().to_string(), formatted_height).await;
    if downscale_res.is_err() {
      return Err(format!("Could not downscale video: {}", downscale_res.unwrap_err()));
    }

    let encode_res = encode_video_to_specific_filesize(project_hash, downscaled_video_path.to_string_lossy().to_string(), output_file, formatted_megabytes.unwrap(), formatted_encoder).await;
    if encode_res.is_err() {
      return Err(format!("Could not re-encode video: {}", encode_res.unwrap_err()));
    }
  } else {
    let downscale_res: Result<(), String> = merge_audios_into_video_and_downscale(project_hash, audio_volumes.len() as i32, start_time, end_time, output_file, formatted_height).await;
    if downscale_res.is_err() {
      return Err(format!("Could not downscale video: {}", downscale_res.unwrap_err()));
     }
  }
  return Ok("Success!".to_owned());
}
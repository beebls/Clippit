// Prevents additional console window on Windows in release, DO NOT REMOVE!!
#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]

mod build_vars;
mod ffmpeg;
mod progress_file;
mod sidecar_wrappers;
mod temp_dirs;
mod user_prefs;
mod winapi;

use std::path::PathBuf;

use ffmpeg::{
    encode_video_to_specific_filesize, extract_video_from_file, format_audio_track,
    get_num_audio_tracks, merge_audios_into_video_and_downscale, split_out_single_audio_track,
};
use progress_file::{write_export_progress_file, write_import_progress_file};
use temp_dirs::{
    clear_temp_dir, create_temp_dir, delete_file, delete_temp_dir, get_output_dir,
    get_project_temp_dir,
};
use user_prefs::get_accent_color as get_platform_accent_color;

fn main() {
    tauri::Builder::default()
        .invoke_handler(tauri::generate_handler![
            start_project,
            export_project,
            get_accent_color,
            remove_project
        ])
        .setup(|_app| {
            // Clear the temp dir
            // Potentially should move this to on close (window-close-requested)
            tauri::async_runtime::spawn(async move {
                let _ = clear_temp_dir().await;
            });
            Ok(())
        })
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}

#[tauri::command]
async fn get_accent_color() -> [u8; 4] {
    get_platform_accent_color().await
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

    write_import_progress_file(temp_hash.to_owned(), "Extracting Video".to_owned(), 1).await;
    match extract_video_from_file(file_name.to_owned(), &full_temp_path).await {
        Ok(_) => {}
        Err(error) => return Err(format!("Error Extracting Video: {}", error)),
    }

    let mut acc: i32 = 0;
    while acc < num_audio_tracks {
        println!("Creating audio file {}", acc);
        write_import_progress_file(
            temp_hash.to_owned(),
            format!("Extracting Audio Track {}", acc + 1),
            acc + 2,
        )
        .await;
        match split_out_single_audio_track(acc, file_name.to_owned(), &full_temp_path).await {
            Ok(_) => acc += 1,
            Err(error) => return Err(format!("Error Extracting Audio Track {}: {}", acc, error)),
        };
    }
    println!(
        "Created temp dir with hash {}, and the audio file has {} tracks",
        temp_hash, num_audio_tracks
    );
    println!("output dir {}", full_temp_path.to_string_lossy());
    write_import_progress_file(temp_hash.to_owned(), "Done".to_owned(), acc + 2).await;
    Ok((num_audio_tracks, temp_hash))
}

// "new" values are set to 0 if they aren't meant to be changed;
#[tauri::command]
async fn export_project(
    project_hash: String,
    start_time: f32,
    end_time: f32,
    audio_volumes: Vec<f32>,
    output_file: String,
    new_height: i32,
    new_megabytes: f32,
    new_aspect: f32,
    encoder_type: String,
) -> Result<String, String> {
    let output_dir_res = get_output_dir(project_hash.to_owned()).await;
    if output_dir_res.is_none() {
        return Err("Cannot create temp output dir".to_owned());
    }
    let output_dir = output_dir_res.unwrap();

    for (index, element) in audio_volumes.iter().enumerate() {
        write_export_progress_file(
            project_hash.to_owned(),
            format!("Format audio track {}", index),
            1.0,
        )
        .await;
        let res = format_audio_track(
            index as i32,
            project_hash.to_owned(),
            element.to_owned(),
            start_time,
            end_time,
        )
        .await;
        if res.is_err() {
            return Err(format!(
                "Error formatting audio track {}: {}",
                index,
                res.unwrap_err()
            ));
        }
    }

    let formatted_height: Option<i32>;
    if new_height == 0 {
        formatted_height = None;
        write_export_progress_file(
            project_hash.to_owned(),
            "Do Not Rescale Video".to_owned(),
            2.0,
        )
        .await;
    } else {
        write_export_progress_file(
            project_hash.to_owned(),
            format!("Rescale video to {}p", new_height),
            2.0,
        )
        .await;
        formatted_height = Some(new_height);
    }

    let formatted_megabytes: Option<f32>;
    if new_megabytes == 0.0 {
        formatted_megabytes = None;
        write_export_progress_file(
            project_hash.to_owned(),
            "Do Not Re-encode Video".to_owned(),
            2.1,
        )
        .await;
    } else {
        formatted_megabytes = Some(new_megabytes);
        write_export_progress_file(
            project_hash.to_owned(),
            format!("Re-encode video to {}MB", new_megabytes),
            2.1,
        )
        .await;
    }

    let formatted_aspect: Option<f32>;
    if new_aspect == 0.0 {
        formatted_aspect = None;
        write_export_progress_file(
            project_hash.to_owned(),
            "Keep same aspect ratio.".to_owned(),
            2.2,
        )
        .await;
    } else {
        formatted_aspect = Some(new_aspect);
        write_export_progress_file(
            project_hash.to_owned(),
            format!("New aspect ratio of {}", new_aspect),
            2.2,
        )
        .await;
    }

    let mut formatted_encoder: String = "x264".to_owned();

    let x264_values = ["h264".to_owned(), "x264".to_owned(), "libx264".to_owned()];
    let x265_values = ["h265".to_owned(), "x265".to_owned(), "libx265".to_owned()];
    if x265_values.contains(&encoder_type) {
        formatted_encoder = "x265".to_owned();
    } else if x264_values.contains(&encoder_type) {
        formatted_encoder = "x264".to_owned();
    }
    write_export_progress_file(
        project_hash.to_owned(),
        format!("Use encoder type {}", formatted_encoder),
        2.2,
    )
    .await;

    if formatted_megabytes.is_some() {
        let downscaled_video_path: PathBuf = output_dir.join("downscaled.mp4");

        write_export_progress_file(
            project_hash.to_owned(),
            "Merging Audio and Downscaling Video".to_owned(),
            3.0,
        )
        .await;
        let downscale_res: Result<(), String> = merge_audios_into_video_and_downscale(
            project_hash.to_owned(),
            audio_volumes.len() as i32,
            start_time,
            end_time,
            downscaled_video_path.to_string_lossy().to_string(),
            formatted_height,
            formatted_aspect,
        )
        .await;
        if downscale_res.is_err() {
            return Err(format!(
                "Could not downscale video: {}",
                downscale_res.unwrap_err()
            ));
        }

        write_export_progress_file(project_hash.to_owned(), "Re-encoding Video".to_owned(), 4.0)
            .await;

        // Old
        let encode_res = encode_video_to_specific_filesize(
            project_hash.to_owned(),
            downscaled_video_path.to_string_lossy().to_string(),
            output_file,
            formatted_megabytes.unwrap(),
            formatted_encoder,
        )
        .await;
        if encode_res.is_err() {
            return Err(format!(
                "Could not re-encode video: {}",
                encode_res.unwrap_err()
            ));
        }
    } else {
        write_export_progress_file(
            project_hash.to_owned(),
            "Merging Audio and Downscaling Video".to_owned(),
            3.0,
        )
        .await;
        let downscale_res: Result<(), String> = merge_audios_into_video_and_downscale(
            project_hash.to_owned(),
            audio_volumes.len() as i32,
            start_time,
            end_time,
            output_file,
            formatted_height,
            formatted_aspect,
        )
        .await;
        if downscale_res.is_err() {
            return Err(format!(
                "Could not downscale video: {}",
                downscale_res.unwrap_err()
            ));
        }
    }
    write_export_progress_file(project_hash.to_owned(), "Done!".to_owned(), 5.0).await;
    Ok("Success!".to_owned())
}

#[tauri::command]
async fn remove_project(file_name: String, project_hash: String) -> Result<(), String> {
    let dir_res: Result<(), String> = delete_temp_dir(project_hash).await;
    if dir_res.is_err() {
        return Err(String::from("Error Removing Temp Dir"));
    }

    let file_res: Result<(), String> = delete_file(file_name).await;
    if file_res.is_err() {
        return Err(String::from("Error Removing Original File"));
    }

    Ok(())
}

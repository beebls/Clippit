use crate::sidecar_wrappers::{add_raw_arg, ffmpeg_command, ffprobe_command};
use crate::temp_dirs::{get_output_dir, get_project_temp_dir};

use std::path::{Path, PathBuf};
use std::process::Command;

pub async fn extract_video_from_file(input_file: String, output_dir: &Path) -> Result<(), String> {
    let command_output: std::process::Output = ffmpeg_command()
        .args([
            "-i",
            &input_file,
            "-map",
            "0:v:0",
            "-c",
            "copy",
            "-y",
            &output_dir.join("video.mp4").to_string_lossy(),
        ])
        .output()
        .expect("Failed to execute the process");

    if command_output.status.success() {
        return Ok(());
    }
    Err(format!("ERROR: {}", command_output.status))
}

pub async fn split_out_single_audio_track(
    track_num: i32,
    input_file: String,
    output_dir: &Path,
) -> Result<(), String> {
    let command_output: std::process::Output = ffmpeg_command()
        .args([
            "-i",
            &input_file,
            "-map",
            &format!("0:a:{}", track_num),
            "-y",
            &output_dir
                .join(format!("track_{}.mp3", track_num))
                .to_string_lossy(),
        ])
        .output()
        .expect("Failed to execute the process");

    if command_output.status.success() {
        return Ok(());
    }
    Err(format!("ERROR: {}", command_output.status))
}

pub async fn get_num_audio_tracks(file_name: String) -> Result<i32, String> {
    let command_output: std::process::Output = ffprobe_command()
        .args([
            "-v",
            "error",
            "-select_streams",
            "a",
            "-show_entries",
            "stream=index",
            "-of",
            "csv=p=0",
            &file_name,
        ])
        .output()
        .expect("Failed to execute the process");

    if command_output.status.success() {
        let stdout: std::borrow::Cow<'_, str> = String::from_utf8_lossy(&command_output.stdout);
        let num_lines: i32 = stdout.split('\n').collect::<Vec<&str>>().len() as i32 - 1;
        return Ok(num_lines);
    }

    Err(format!("ERROR: {}", command_output.status))
}

pub async fn format_audio_track(
    track_num: i32,
    project_hash: String,
    volume: f32,
    start_time: f32,
    end_time: f32,
) -> Result<(), String> {
    let temp_path_res: Option<PathBuf> = get_project_temp_dir(project_hash.to_owned()).await;
    if temp_path_res.is_none() {
        return Err("Cannot get project dir".to_owned());
    }
    let track_file_name: String = format!("track_{}.mp3", track_num);

    let full_temp_path: PathBuf = temp_path_res.unwrap();
    let input_path: PathBuf = full_temp_path.join(&track_file_name);
    let output_path: PathBuf = full_temp_path.join("output").join(&track_file_name);

    let command_output: std::process::Output = ffmpeg_command()
        .args([
            "-ss",
            &start_time.to_string(),
            "-to",
            &end_time.to_string(),
            "-i",
            &input_path.to_string_lossy(),
            "-filter:a",
            &format!("volume={}", volume),
            "-y",
            &output_path.to_string_lossy(),
        ])
        .output()
        .expect("Failed to execute the process");

    if command_output.status.success() {
        return Ok(());
    }

    Err(format!(
        "ERROR: {} \n {}",
        command_output.status,
        String::from_utf8_lossy(&command_output.stderr)
    ))
}

pub async fn merge_audios_into_video_and_downscale(
    project_hash: String,
    num_audio_files: i32,
    start_time: f32,
    end_time: f32,
    output_path: String,
    new_height: Option<i32>,
    new_aspect: Option<f32>,
) -> Result<(), String> {
    let proj_dir_res: Option<PathBuf> = get_project_temp_dir(project_hash.to_owned()).await;
    if proj_dir_res.is_none() {
        return Err("Can't get project dir".to_owned());
    }
    let full_temp_path: PathBuf = proj_dir_res.unwrap();
    let output_res: Option<PathBuf> = get_output_dir(project_hash).await;
    if output_res.is_none() {
        return Err("Can't get output dir".to_owned());
    }
    let temp_output_path: PathBuf = output_res.unwrap();

    let video_path: PathBuf = full_temp_path.join("video.mp4");

    let mut audio_mux_string: String = String::from("");

    let mut command: Command = ffmpeg_command();

    command.arg("-ss");
    command.arg(start_time.to_string());
    command.arg("-to");
    command.arg(end_time.to_string());
    command.arg("-i");
    command.arg(video_path.to_string_lossy().to_string());

    let mut acc: i32 = 0;
    while acc < num_audio_files {
        audio_mux_string.push_str(&format!("[{}:a]", acc + 1));
        let file_path: PathBuf = temp_output_path.join(format!("track_{}.mp3", acc));

        let file_str: String = file_path.to_string_lossy().to_string();
        command.arg("-i");
        command.arg(file_str);
        acc += 1;
    }
    audio_mux_string.push_str(&format!("amix=inputs={}[a]", num_audio_files));
    if new_height.is_some() || new_aspect.is_some() {
        command.arg("-vf");
        let mut formatted_height: String = "ih".to_owned();
        if new_height.is_some() {
            formatted_height = new_height.unwrap().to_string();
        }

        let mut formatted_aspect: String = "a".to_owned();
        if new_aspect.is_some() {
            formatted_aspect = new_aspect.unwrap().to_string();
        }

        let scale_string = format!(
            "scale=\"trunc(oh*{}/2)*2:{}\",setsar=1",
            formatted_aspect, formatted_height
        );
        println!("{}", scale_string);
        command = add_raw_arg(command, scale_string);
    }
    command.arg("-filter_complex");
    command.arg(audio_mux_string);
    command.arg("-map");
    command.arg("0");
    command.arg("-map");
    command.arg("[a]");
    command.arg("-y");
    command.arg(output_path);

    println!("{:?}", command.get_args());

    let command_output: std::process::Output = command.output().expect("Failed to execute process");
    if command_output.status.success() {
        return Ok(());
    }

    Err(format!("ERROR: {}", command_output.status))
}

pub async fn encode_video_to_specific_filesize(
    project_hash: String,
    input_file: String,
    output_file: String,
    target_mebibytes: f32,
    encoder_type: String,
) -> Result<(), String> {
    let output_res: Option<PathBuf> = get_output_dir(project_hash).await;
    if output_res.is_none() {
        return Err("Can't get output dir".to_owned());
    }
    let temp_output_path: PathBuf = output_res.unwrap();

    let duration_cmd: std::process::Output = ffprobe_command()
        .args([
            "-v",
            "error",
            "-show_entries",
            "format=duration",
            "-of",
            "csv=p=0",
            &input_file,
        ])
        .output()
        .expect("Failed to execute process");
    let duration_str: String = String::from_utf8_lossy(&duration_cmd.stdout).to_string();
    let duration: f32 = duration_str.trim().parse::<f32>().unwrap();
    // let err: std::borrow::Cow<'_, str> = String::from_utf8_lossy(&value.stderr);
    // let num_lines: i32 = out.split("\n").collect::<Vec<&str>>().len() as i32 - 1;
    println!("Duration {}", duration);

    let audio_rate_cmd: std::process::Output = ffprobe_command()
        .args([
            "-v",
            "error",
            "-select_streams",
            "a:0",
            "-show_entries",
            "stream=bit_rate",
            "-of",
            "csv=p=0",
            &input_file,
        ])
        .output()
        .expect("Failed to execute process");
    let audio_rate_str: String = String::from_utf8_lossy(&audio_rate_cmd.stdout).to_string();
    // This command prints out bits PER SECOND
    // we need to convert this to MEBIBITS per second
    let audio_rate_mebibits: f32 = audio_rate_str.trim().parse::<f32>().unwrap() / 1048576.0;

    if target_mebibytes < audio_rate_mebibits / 8.0 * duration {
        // ERROR: IDK
        println!(
            "test {} {}",
            audio_rate_mebibits,
            audio_rate_mebibits / 8.0 * duration
        );
        return Err("Target size smaller than just the audio alone.".to_owned());
    }

    // this is to ensure you dont go above the original;
    const EXTRA_PADDING_MEBIBYTES: f32 = 1.0;

    let video_rate_mebibytes =
        (target_mebibytes - EXTRA_PADDING_MEBIBYTES - (audio_rate_mebibits / 8.0 * duration))
            / duration;

    let mut pass_one = ffmpeg_command();
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
    let pass_one_output: std::process::Output =
        pass_one.output().expect("Failed to execute process");
    if !pass_one_output.status.success() {
        println!("{}", String::from_utf8_lossy(&pass_one_output.stderr));
        return Err(format!("ERROR During Pass One: {}", pass_one_output.status));
    }

    let mut pass_two = ffmpeg_command();
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

    let pass_two_output: std::process::Output =
        pass_two.output().expect("Failed to execute process");
    if !pass_two_output.status.success() {
        return Err(format!("ERROR During Pass Two: {}", pass_two_output.status));
    }

    Ok(())
}

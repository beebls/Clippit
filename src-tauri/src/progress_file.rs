use std::fs;
use crate::temp_dirs::{get_project_temp_dir, get_output_dir};

pub async fn write_import_progress_file(project_hash: String, content: String, number: i32) -> Option<()> {
  let project_dir = get_project_temp_dir(project_hash).await;
  if project_dir.is_none() {
    return None;
  }
  let path = project_dir.unwrap().join("import_progress.json");

  let mut json = r#"{"number": "#.to_owned();
  json.push_str(&format!("{},", number));
  json.push_str(r#""text": ""#);
  json.push_str(&format!("{}\"}}", content));

  let fs_res = fs::write(path, json);
  if fs_res.is_err() {
    return None;
  }
  return Some(());
}

pub async fn write_export_progress_file(project_hash: String, content: String, number: f32) -> Option<()> {
  let output_dir = get_output_dir(project_hash).await;
  if output_dir.is_none() {
    return None;
  }
  let path = output_dir.unwrap().join("export_progress.json");

  let mut json = r#"{"number": "#.to_owned();
  json.push_str(&format!("{},", number));
  json.push_str(r#""text": ""#);
  json.push_str(&format!("{}\"}}", content));

  let fs_res = fs::write(path, json);
  if fs_res.is_err() {
    return None;
  }
  return Some(());
}
use crate::temp_dirs::{get_output_dir, get_project_temp_dir};
use std::fs;

pub async fn write_import_progress_file(
    project_hash: String,
    content: String,
    number: i32,
) -> Option<()> {
    let project_dir = get_project_temp_dir(project_hash).await;
    project_dir.as_ref()?;

    let path = project_dir.unwrap().join("import_progress.json");

    let mut json = r#"{"number": "#.to_owned();
    json.push_str(&format!("{},", number));
    json.push_str(r#""text": ""#);
    json.push_str(&format!("{}\"}}", content));

    let fs_res = fs::write(path, json);

    if fs_res.is_err() {
        return None;
    }
    Some(())
}

pub async fn write_export_progress_file(
    project_hash: String,
    content: String,
    number: f32,
) -> Option<()> {
    let output_dir = get_output_dir(project_hash).await;

    // New
    output_dir.as_ref()?;

    let path = output_dir.unwrap().join("export_progress.json");

    let mut json = r#"{"number": "#.to_owned();
    json.push_str(&format!("{},", number));
    json.push_str(r#""text": ""#);
    json.push_str(&format!("{}\"}}", content));

    let fs_res = fs::write(path, json);
    if fs_res.is_err() {
        return None;
    }
    Some(())
}

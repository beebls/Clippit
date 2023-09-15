use directories::BaseDirs;
use std::collections::hash_map::DefaultHasher;
use std::hash::{Hash, Hasher};
use std::path::{Path,PathBuf};
use std::fs;

pub async fn get_temp_root() -> Option<PathBuf> {
  if let Some(base_dirs) = BaseDirs::new() {
    let cache_root: &Path = base_dirs.cache_dir();
    let temp_root: PathBuf = cache_root.join("com.beebles.clippit").join("temp");
    if !temp_root.exists() {
      println!("Temp Dir does not exist, creating");
      fs::create_dir_all(&temp_root).expect("Failed to create dir");
    }
    return Some(temp_root);
  }
  return None;
}

pub async fn get_project_temp_dir(project_hash: String) -> Option<PathBuf> {
  let temp_root = get_temp_root().await;
  if temp_root.is_none() {
    return None;
  }
  let project_dir = temp_root.unwrap().join(project_hash);
  if !project_dir.exists() {
    println!("Project Dir does not exist, creating");
    fs::create_dir_all(&project_dir).expect("Failed to create dir");
  }
  return Some(project_dir);
}

pub async fn create_temp_dir(file_name: String) -> String {
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

pub async fn get_output_dir(project_hash: String) -> Option<PathBuf> {
  let proj_dir = get_project_temp_dir(project_hash).await;
  if proj_dir.is_none() {
    return None;
  }
  let output_dir = proj_dir.unwrap().join("output");
  let _ = fs::create_dir_all(&output_dir);
  return Some(output_dir);
}
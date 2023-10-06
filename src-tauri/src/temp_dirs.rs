use directories::BaseDirs;
use std::collections::hash_map::DefaultHasher;
use std::fs;
use std::hash::{Hash, Hasher};
use std::path::{Path, PathBuf};

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
    None
}

pub async fn get_project_temp_dir(project_hash: String) -> Option<PathBuf> {
    let temp_root = get_temp_root().await;
    temp_root.as_ref()?;
    let project_dir = temp_root.unwrap().join(project_hash);
    if !project_dir.exists() {
        println!("Project Dir does not exist, creating");
        fs::create_dir_all(&project_dir).expect("Failed to create dir");
    }
    Some(project_dir)
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
    hash
}

pub async fn get_output_dir(project_hash: String) -> Option<PathBuf> {
    let proj_dir = get_project_temp_dir(project_hash).await;
    proj_dir.as_ref()?;
    let output_dir = proj_dir.unwrap().join("output");
    let _ = fs::create_dir_all(&output_dir);
    Some(output_dir)
}

pub async fn delete_temp_dir(project_hash: String) -> Result<(), String> {
    let proj_dir = get_project_temp_dir(project_hash).await;
    if proj_dir.is_none() {
        return Err(String::from("Cannot find project dir"));
    }
    let remove_res: Result<(), std::io::Error> = fs::remove_dir_all(proj_dir.unwrap());
    if remove_res.is_err() {
        return Err(String::from("Error Removing Folder"));
    }
    Ok(())
}

pub async fn delete_file(file_path: String) -> Result<(), String> {
    let path: PathBuf = PathBuf::from(file_path);

    // If the file doesn't exist, it technically is already removed
    let exists: Result<bool, std::io::Error> = path.try_exists();
    if exists.is_err() || !exists.unwrap() {
        return Ok(());
    }

    let remove_res: Result<(), std::io::Error> = fs::remove_file(path);
    if remove_res.is_err() {
        return Err(String::from("Error Removing File"));
    }
    Ok(())
}

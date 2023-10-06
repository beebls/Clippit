#[cfg(target_os = "windows")]
use crate::winapi::get_windows_accent_color;

#[cfg(target_os = "windows")]
pub async fn get_accent_color() -> [u8; 4] {
  let res: Result<[u8; 4], String> = get_windows_accent_color().await;
  if res.is_err() {
    return [255, 0, 0, 0];
  }
  res.unwrap()
}

#[cfg(target_os = "linux")]
pub async fn get_accent_color() -> [u8; 4] {
  return [255, 0, 0, 0];
}
pub fn is_using_sidecar() -> bool {
  if cfg!(windows) {
    return false;
  } else if cfg!(unix) {
    return false;
  }
  return false;
}
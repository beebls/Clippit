pub fn is_using_sidecar() -> bool {
  if cfg!(windows) {
    return true;
  } else if cfg!(unix) {
    return false;
  }
  return false;
}
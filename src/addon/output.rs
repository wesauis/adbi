use std::process::Output;
use super::vecu8::ToUTF8Lossy;

pub trait AsResult<T, E> {
  fn as_result(self) -> Result<T, E>;
}

impl AsResult<String, String> for Output {
  fn as_result(self) -> Result<String, String> {
    if self.status.success() {
      Ok(self.stdout.to_utf8_lossy())
    } else {
      Err(self.stderr.to_utf8_lossy())
    }
  }
}
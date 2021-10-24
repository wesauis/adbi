pub trait ToUTF8Lossy {
  fn to_utf8_lossy(&self) -> String;
}

impl ToUTF8Lossy for Vec<u8> {
  fn to_utf8_lossy(&self) -> String {
    String::from_utf8_lossy(self).to_string()
  }
}

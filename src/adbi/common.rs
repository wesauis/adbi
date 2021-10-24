use std::env;


pub fn sdk_root() -> Result<String, String> {
  Ok(env::var("ANDROID_SDK_ROOT")
    .map_err(|_| "ANDROID_SDK_ROOT isn't set or invalid")?)
}

pub fn adb_bin() -> Result<String, String> {
  let sdk_root = sdk_root()?;

  Ok(if env::consts::OS == "windows" {
    format!("{}/platform-tools/adb.exe", sdk_root)
  } else {
    format!("{}/platform-tools/adb", sdk_root)
  })
}

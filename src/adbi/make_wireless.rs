use std::process::Command;

use regex::Regex;

use crate::addon::output::AsResult;

pub fn get_device_ip(adb_bin: &str) -> Result<String, String> {
  // adb shell ip addr show wlan0
  let output = Command::new(adb_bin)
    .args(&["shell","ip","addr","show","wlan0"])
    .output()
    .map_err(|_| "unable to run adb, the executable exists?")?
    .as_result()?;

  let re_ip = Regex::new("inet (\\d{1,3}\\.\\d{1,3}\\.\\d{1,3}\\.\\d{1,3})/")
    .unwrap();

  Ok(String::from(re_ip
    .captures(&output)
    .and_then(|captures| captures.get(1))
    .map(|group| group.as_str())
    .ok_or_else(|| "unable to device's ip")?))
}
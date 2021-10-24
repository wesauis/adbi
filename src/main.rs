mod log;
mod addon;
mod adbi;

use std::process::Command;

use colored::*;
use crate::adbi::common::adb_bin;
use crate::adbi::make_wireless::get_device_ip;
use crate::log::info;


fn main() {
  log::init();

  if let Err(msg) = adbi() {
    log::error(&msg);
    std::process::exit(1);
  }
}

fn adbi() -> Result<(), String> {
  let port = 55555;

  let adb_bin = adb_bin()?;
  let device_ip = get_device_ip(&adb_bin)?;
  info(&format!("device found at {}", device_ip));

  // adb tcpip $PORT
  if !Command::new(&adb_bin)
      .args(&["tcpip", &55555.to_string()])
      .output()
      .map(|output| output.status.success())
      .map_err(|_| "unable to run adb, the executable exists?")? {
    return Err(format!("unable to start adb at tcpip mode at port {}", port));
  }
  info(&format!("started adb in tcpip mode at port {}", port));

  // adb connect "$IP:$PORT"
  if !Command::new(&adb_bin)
      .args(&["connect", &format!("{}:{}", device_ip, port)])
      .output()
      .map(|output| output.status.success())
      .map_err(|_| "unable to run adb, the executable exists?")? {
    return Err(format!("unable connect to device at {}:{}", device_ip, port));
  }
  info(&format!("connected to {}:{}", device_ip, port));

  println!("{}", " you can detach your device now! ".black().on_cyan());
  Ok(())
}




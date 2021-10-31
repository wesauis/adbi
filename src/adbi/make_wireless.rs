use color_eyre::{
    eyre::{eyre, ContextCompat},
    Result,
};
use regex::Regex;

use crate::adbi::common::adb_exec;
use crate::addon::output::AsResult;

pub fn get_device_ip() -> Result<String> {
    // adb shell ip addr show wlan0
    let output = adb_exec(&["shell", "ip", "addr", "show", "wlan0"])?
        .as_result()
        .map_err(|stderr| eyre!(stderr))?;

    let re_ip = Regex::new("inet (\\d{1,3}\\.\\d{1,3}\\.\\d{1,3}\\.\\d{1,3})/").unwrap();

    Ok(String::from(
        re_ip
            .captures(&output)
            .and_then(|captures| captures.get(1))
            .map(|group| group.as_str())
            .wrap_err("unable to get device's ip")?,
    ))
}

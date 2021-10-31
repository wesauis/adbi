use color_eyre::{eyre::eyre, Result};
use colored::*;

mod adbi;
mod addon;

use crate::adbi::common::adb_exec;
use crate::adbi::make_wireless::get_device_ip;

fn main() -> Result<()> {
    color_eyre::install()?;

    adbi()?;
    Ok(())
}

fn adbi() -> Result<()> {
    let port = 55555;

    let device_ip = get_device_ip()?;
    println!("device found at {}", device_ip);

    // adb tcpip $PORT
    if !adb_exec(&["tcpip", &port.to_string()])?
        .status
        .success()
    {
        return Err(eyre!(format!(
            "unable to start adb at tcpip mode at port {}",
            port
        )));
    }
    println!("started adb in tcpip mode at port {}", port);

    // adb connect "$IP:$PORT"
    if !adb_exec(&["connect", &format!("{}:{}", device_ip, port)])?
        .status
        .success()
    {
        return Err(eyre!("unable connect to device at {}:{}", device_ip, port));
    }
    println!("connected to {}:{}", device_ip, port);

    // todo!("spantrace");
    // todo!("logging");
    // todo!("opt args");

    println!("{}", " you can detach your device now! ".black().on_cyan());
    Ok(())
}

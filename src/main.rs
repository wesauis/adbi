use color_eyre::{eyre::eyre, Result};
use colored::*;

use crate::adb::ADB;

mod addon;
mod adb;

const PORT: i32 = 55555;

fn main() -> Result<()> {
    color_eyre::install()?;

    let adb = ADB::find()?;

    let device_ip = adb.get_device_ip()?;
    println!("device found at {}", device_ip);

    // adb tcpip $PORT
    if !adb.exec(&["tcpip", &PORT.to_string()])?
        .status
        .success()
    {
        return Err(eyre!("unable to start adb at tcpip mode at port {}", &PORT));
    }
    println!("started adb in tcpip mode at port {}", &PORT);

    // adb connect "$IP:$PORT"
    if !adb.exec(&["connect", &format!("{}:{}", device_ip, &PORT)])?
        .status
        .success()
    {
        return Err(eyre!("unable connect to device at {}:{}", device_ip, &PORT));
    }
    println!("connected to {}:{}", device_ip, &PORT);

    // todo!("spantrace");
    // todo!("logging");
    // todo!("opt args");

    println!("{}", " you can detach your device now! ".black().on_cyan());
    Ok(())
}

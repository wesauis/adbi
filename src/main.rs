use color_eyre::Result;
use colored::*;

mod adb;
use crate::adb::ADB;

mod addon;

const PORT: u16 = 55555;

fn main() -> Result<()> {
    color_eyre::install()?;

    let adb = ADB::find()?;

    let device_ip = adb.get_device_ip()?;
    println!("device found at {}", device_ip);

    adb.listen_tcpip(&PORT)?;
    println!("started adb in tcpip mode at port {}", &PORT);

    adb.remote_connect(&device_ip, &PORT)?;
    println!("connected to {}:{}", device_ip, &PORT);

    // todo!("spantrace");
    // todo!("logging");
    // todo!("opt args");

    println!("{}", " you can detach your device now! ".black().on_cyan());
    Ok(())
}

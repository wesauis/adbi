use std::{
    env,
    ffi::OsStr,
    process::{Command, Output},
};

use color_eyre::{eyre::Context, Result};

pub fn sdk_root() -> Result<String> {
    let sdk_root =
        env::var("ANDROID_SDK_ROOT").wrap_err("ANDROID_SDK_ROOT isn't set or invalid")?;

    return Ok(sdk_root);
}

pub fn adb_bin() -> Result<String> {
    let sdk_root = sdk_root()?;

    Ok(if env::consts::OS == "windows" {
        format!("{}/platform-tools/adb.exe", sdk_root)
    } else {
        format!("{}/platform-tools/adb", sdk_root)
    })
}

pub fn adb_exec<I: IntoIterator<Item = S>, S: AsRef<OsStr>>(args: I) -> Result<Output> {
    let adb_bin = adb_bin()?;

    let output = Command::new(adb_bin)
        .args(args)
        .output()
        .wrap_err("unable to run adb")?;

    Ok(output)
}

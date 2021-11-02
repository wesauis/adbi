use color_eyre::{
    eyre::{eyre, ContextCompat, Context},
    Result,
};
use regex::Regex;
use std::{env, ffi::OsStr, process::{Command, Output}};

use crate::addon::{output::AsResult};

pub struct ADB {
    bin: String,
}

/// impl adb::core::ADB
/// impl adb::tcpip
/// impl adb::connection
impl ADB {
    pub fn find() -> Result<ADB> {
        let sdk_root = ADB::sdk_root()?;
        let adb_bin = ADB::adb_bin(&sdk_root)?;

        let adb = ADB { bin: adb_bin };
        println!("ADB found!\n {}", adb.version()?);

        Ok(adb)
    }

    fn sdk_root() -> Result<String> {
        env::var("ANDROID_SDKa_ROOT").wrap_err("ANDROID_SDK_ROOT isn't set or invalid")
    }

    fn adb_bin(sdk_root: &String) -> Result<String> {
        Ok(if env::consts::OS == "windows" {
            format!("{}/platform-tools/adb.exe", sdk_root)
        } else {
            format!("{}/platform-tools/adb", sdk_root)
        })
    }

    pub fn exec<I: IntoIterator<Item = S>, S: AsRef<OsStr>>(&self, args: I) -> Result<Output> {
        Command::new(&self.bin)
            .args(args)
            .output()
            .wrap_err("unable to run adb")
    }

    pub fn exec_status<I: IntoIterator<Item = S>, S: AsRef<OsStr>>(&self, args: I) -> Result<Result<String, String>> {
        self.exec(args).map(|output| output.as_result())
    }

    fn version(&self) -> Result<String> {
        self.exec_status(&["--version"])?
            .map_err(|err| eyre!(err).wrap_err("Invalid adb binary"))
    }

    pub fn get_device_ip(&self) -> Result<String> {
        let output = self.exec_status(&["shell", "ip", "addr", "show", "wlan0"])?
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
}

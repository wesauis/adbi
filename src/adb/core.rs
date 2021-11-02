
use color_eyre::{
    eyre::{eyre, Context},
    Result,
};
use colored::Colorize;
use std::{
    env,
    process::{Command, Output},
};

use crate::addon::vecu8::ToUTF8Lossy;
use super::ADB;

impl ADB {
    /// Finds the adb executable wrapped into a ADB impl
    pub fn find() -> Result<ADB> {
        let adb_bin = ADB::adb_bin()?;

        let adb = ADB { bin: adb_bin };
        for line in adb
        .version()?
        .split("\n")
        .map(|s| s.trim())
        .filter(|str| str.len() > 0)
        {
            println!("{tag} {}", line, tag = "INFO".cyan());
        }

        Ok(adb)
    }

    /// Returns the path to adb's binary
    fn adb_bin() -> Result<String> {
        let sdk_root =
        env::var("ANDROID_SDK_ROOT").wrap_err("ANDROID_SDK_ROOT isn't set or invalid")?;

        Ok(if env::consts::OS == "windows" {
            format!("{}/platform-tools/adb.exe", sdk_root)
        } else {
            format!("{}/platform-tools/adb", sdk_root)
        })
    }

    /// Executes adb with args
    pub(super) fn exec<I: IntoIterator<Item = S>, S: AsRef<std::ffi::OsStr>>(
        &self,
        args: I,
    ) -> Result<Output> {
        Command::new(&self.bin)
        .args(args)
        .output()
        .wrap_err("unable to run adb")
    }

    /// Returns the output of `adb --version`
    pub fn version(&self) -> Result<String> {
        let output = self.exec(&["--version"])?;

        if !output.status.success() {
            return Err(eyre!(output.stderr.to_utf8_lossy()));
        }

        Ok(output.stdout.to_utf8_lossy())
    }
}


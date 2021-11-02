use color_eyre::{
    eyre::{eyre, ContextCompat},
    Result,
};
use regex::Regex;

use crate::addon::vecu8::ToUTF8Lossy;

use super::ADB;

impl ADB {
    pub fn get_device_ip(&self) -> Result<String> {
        let output = self.exec(&["shell", "ip", "addr", "show", "wlan0"])?;

        if !output.status.success() {
            return Err(eyre!(output.stderr.to_utf8_lossy()));
        }

        let re_ip = Regex::new("inet (\\d{1,3}\\.\\d{1,3}\\.\\d{1,3}\\.\\d{1,3})/").unwrap();

        Ok(String::from(
            re_ip
                .captures(&output.stdout.to_utf8_lossy())
                .and_then(|captures| captures.get(1))
                .map(|group| group.as_str())
                .wrap_err("unable to get device's ip")?,
        ))
    }

    pub fn listen_tcpip(&self, port: &u16) -> Result<()> {
        if !self.exec(&["tcpip", &port.to_string()])?.status.success() {
            return Err(eyre!("unable to start adb at tcpip mode at port {}", port));
        }

        Ok(())
    }

    pub fn remote_connect(&self, ip: &String, port: &u16) -> Result<()> {
        if !self.exec(&["connect", &format!("{}:{}", ip, port)])?.status.success() {
            return Err(eyre!("unable connect to device at {}:{}", ip, port));
        }

        Ok(())
    }
}

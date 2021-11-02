# [Changelog](https://keepachangelog.com)
## [Unreleased]
### TODO
- [ ] update the flow + color_eyre
    - [x] get sdk_root
    - [x] get adb_bin
    - [x] check version and log it
    - [x] start server, if it does, log it
    - [x] enable tcpip mode at port and log it
    - [x] get device ip and log it
    - [x] connect to device
    - [ ] eyre
- [ ] command line arguments
- [ ] debug logs - `DEBUG=1`
    - [ ] ANDROID_SDK_ROOT=""
    - [ ] $log executed command, exit_code=XXX
    - [ ] turn some of the logs of task #1 into debug ones
    - [ ] add "inspect what is happening by enabling debug logs with `DEBUG=1`"

## [0.0.0] - 2020-10-24

This first version allows to connect to a device over tcp by simply running `adbi`

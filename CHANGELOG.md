# [Changelog](https://keepachangelog.com)
## [Unreleased]
### TODO
- [ ] update the flow + use color_eyre
    - [ ] get sdk_root
    - [ ] get adb_bin
    - [ ] check version and log it
    - [ ] start server, if it does, log it
    - [ ] enable tcpip mode at port and log it
    - [ ] get device ip and log it
    - [ ] connect to device
- [ ] command line arguments
- [ ] debug logs - `DEBUG=1`
    - [ ] ANDROID_SDK_ROOT=""
    - [ ] $log executed command, exit_code=XXX
    - [ ] turn some of the logs of task #1 into debug ones
    - [ ] add "inspect what is happening by enabling debug logs with `DEBUG=1`"

## [0.0.0] - 2020-10-24

This first version allows to connect to a device over tcp by simply running `adbi`

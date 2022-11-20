# Machine Monitor

Executable binary and installation (.msi) exist in releases --->

View temperature and other information about the current machine.

Only Windows is supported and NVidia card for graphics information

![image](app.gif)

## Compilation
This project is built with Tauri, to compile it follow the setup steps for tauri using `cargo` on https://tauri.app/.
To build a development build use the following command:
```$ cargo tauri dev```
To build a release build use the following:
```$ cargo tauri build```

## Back-end
Much of the back-end is handled by my qmstats crate which can be found at: https://github.com/AlbinDalbert/qmstats

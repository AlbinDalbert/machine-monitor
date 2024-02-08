# Machine Monitor
This is a minimal monitor program which has the goal to use as little resources as possible
View temperature and other information about the current machine.

Executable binary and installation (.msi) exist in releases --->

Only Windows is supported and Nvidia for graphics information

![image](app.gif)

## Hardware Limitation
This program is specifically designed to work on windows machines with Nvidia graphicscards.
This might expand in the future.

## Summary

### Front-End
This program is created using the Rust framework `Tauri` using a pure HTML, CSS, JS implementation.

### Back-End
The back-end uses multithreading. One thread executing all the communication with the Windows and NVidia APIs getting all data for the frontend. And the main rust thread taking the data from the APIs and sending it to the front-end.

Much of the back-end implementation is implemented in my rust crate `qmstats` which can be found at: https://github.com/AlbinDalbert/qmstats
## Compilation
This project is built with Tauri, to compile it follow the setup steps for tauri using `cargo` at https://tauri.app/.
To build a development build use the following command:

```$ cargo tauri dev```

To build a release build use the following:

```$ cargo tauri build```

### TODO
[ ] logging (when installed, not when binary)
[ ] add config file
[ ] posibility to change refresh rate
[ ] about page
[ ] display version in UI



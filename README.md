# Example of integration ESP-IDF and Rust NMEA crate

The example shows how to integrate ESP-IDF with Rust no_std crate like [NMEA](https://crates.io/crates/nmea).

## Prerequisites

- [Installed ESP-IDF](https://docs.espressif.com/projects/esp-idf/en/latest/esp32/get-started/index.html#installation)
- [Installed Rust and Cargo](https://www.rust-lang.org/tools/install)
- [Installed Xtensa LLVM toolchain for Rust](https://esp-rs.github.io/book/installation/index.html)
- Basic knowledge of ESP-IDF, CMake, and Rust

## Structure

Here's how your project directory might look after the following the guide:

```
esp_idf_project/
|-- CMakeLists.txt
|-- main/
|   |-- CMakeLists.txt
|   |-- esp_idf_project.c
|-- sdkconfig
|-- components/
|   |-- nma /
|       |-- CMakeLists.txt
|       |-- include/
|           |-- nmea_idf.h
|       |-- nmea_idf.c
|       |-- nmea_idf_rs /
|           |-- Cargo.toml
|           |-- rust-toolchain.toml
|           |-- src/
|               |-- lib.rs
```

### Architecture

Key elements:
- `esp_idf_project` contains main C code like any other ESP-IDF application.
- The ESP-IDF componet with name `nmea` is stored in subdirectory with components.
  - The `nmea_idf_rs` component contains C adapter layer which helps interfacing with Rust library.
- The Rust code is stored in `components/nmea/nmea_idf_rs` subdirectory.

The component can be uploaded later on to [Component Manager](https://components.espressif.com/).


### Select target

Set target for main ESP-IDF application:

```bash
#idf.py set-target <target>
# idf.py set-target esp32
idf.py set-target esp32-c3
# idf.py set-target esp32-s3
```

Optional step when developers needs to build Rust component also manually:
Define which toolchain should be used for the Rust component in file `esp_rust_component/rust_crate/rust-toolchain.toml`

```toml
[toolchain]
# Use "esp" for ESP32, ESP32-S2, and ESP32-S3
#channel = "esp"
# Use "nightly" for ESP32-C*, ESP32-H* targets
channel = "nightly"
```

### Build the Project
From the base folder of the project (`esp_idf_project`), run the build process as you usually would for an ESP-IDF project:

```bash
idf.py build flash monitor
```

### Expected output

```
I (316) main_task: Started on CPU0
I (316) main_task: Calling app_main()
Remaining stack space before calling Rust function in app_main: 2392
Message from Rust: Hello ESP-RS. https://github.com/esp-rs
Size of NMEA: 12272
Remaining stack space after calling Rust function in app_main: 2392
Remaining stack space before calling Rust function in nmea_gga_task: 36068
NMEA altitude: 61.700001
Remaining stack space after calling Rust function in nmea_gga_task: 25924
GGA Data:
- Latitude: 53°21.6802' N
- Longitude: -7°29.6628' W
- GPS Quality: 1 (GPS fix)
- Number of Satellites: 8
- Horizontal Dilution of Precision: 1.0
- Altitude: 61.7 Meters
- Height of Geoid above WGS84 Ellipsoid: 55.2 Meters
I (376) main_task: Returned from app_main()
```

## Simulation

### Simulation with Wokwi in VS Code

[Wokwi for Visual Studio Code](https://docs.wokwi.com/vscode/getting-started) provides a simulation solution for embedded and IoT system engineers. The extension integrates with your existing development environment, allowing you to simulate your projects directly from your code editor.

- [Install VS Code](https://code.visualstudio.com/download)
- [Install Wokwi plugin](https://docs.wokwi.com/vscode/getting-started#installation)
- Activate Wokwi plugin - command palette, search for `Wokwi: Start Simulator`, select and ativate the plugin using web browser


# Example of integration ESP-IDF and Rust NMEA crate

The example shows how to integrate ESP-IDF with Rust no_std crate like NMEA.

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
|   |-- esp_rust_component/
|       |-- CMakeLists.txt
|       |-- include/
|           |-- esp_rust_component.h
|       |-- esp_rust_component.c
|       |-- rust_crate/
|           |-- Cargo.toml
|           |-- rust-toolchain.toml
|           |-- src/
|               |-- lib.rs
```

### Architecture

Key elements:
- `esp_idf_project` contains main C code like any other ESP-IDF application.
- The ESP-IDF componet with name `esp_rust_component` is stored in subdirectory with components.
  - The `esp_rust_component` component contains C adapter layer which helps interfacing with Rust library.
- The Rust code is stored in `components/esp_rust_component/rust_crate` subdirectory.

The component can be uploaded later on to [Component Manager](https://components.espressif.com/).


### Select target

Set target for main ESP-IDF application:

```bash
idf.py set-target <target>
# idf.py set-target esp32
# idf.py set-target esp32-c3
# idf.py set-target esp32-s3
```

Optional step when developers needs to build Rust component also manually:
Define which toolchain should be used for the Rust component in file `esp_rust_component/rust_crate/rust-toolchain.toml`

```toml
[toolchain]
# Use "esp" for ESP32, ESP32-S2, and ESP32-S3
channel = "esp"
# Use "nightly" for ESP32-C*, ESP32-H* targets
# channel = "nightly"

```


### Build the Project
From the base folder of the project (`esp_idf_project`), run the build process as you usually would for an ESP-IDF project:

```bash
idf.py build flash monitor
```


## Simulation

### Simulation with Wokwi in VS Code

[Wokwi for Visual Studio Code](https://docs.wokwi.com/vscode/getting-started) provides a simulation solution for embedded and IoT system engineers. The extension integrates with your existing development environment, allowing you to simulate your projects directly from your code editor.

- [Install VS Code](https://code.visualstudio.com/download)
- [Install Wokwi plugin](https://docs.wokwi.com/vscode/getting-started#installation)
- Activate Wokwi plugin - command palette, search for `Wokwi: Start Simulator`, select and ativate the plugin using web browser


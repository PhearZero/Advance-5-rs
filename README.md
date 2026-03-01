# CrowPanel Advance 5.0 Rust Library

A Rust library for the **Elecrow CrowPanel Advance 5.0 HMI (Human Machine Interface)**. This project is a rewrite into Rust of the original C++ libraries, enabling high-performance, safe, and modern development for the ESP32-S3-based display.

## Overview

The CrowPanel Advance 5.0 is an AI-powered IPS touch screen based on the ESP32-S3. This library provides low-level and high-level abstractions to interact with the hardware components:
- **RGB LCD Display**: 800x480 resolution with `embedded-graphics` support.
- **GT911 Touch Controller**: Capacitive touch screen support.
- **STC8H1K28 Co-processor**: Controls backlight, buzzer, and audio muting.
- **BM8563 RTC**: High-precision real-time clock.
- **Wireless Module Port**: Support for LoRa/Wireless modules on the expansion port.

## Device Specifications

- **MCU**: ESP32-S3-WROOM-1-N16R8 (Dual-core, 240MHz, 16MB Flash, 8MB PSRAM).
- **Display**: 5.0-inch IPS, 800x480 resolution, 16-bit RGB interface.
- **Touch**: GT911 Capacitive Touch.
- **Audio**: Built-in 2W speaker and 1W speaker connector.
- **Connectivity**: Wi-Fi, Bluetooth 5.0, USB Type-C, TF Card slot, expansion headers (I2C, UART).

For more hardware details, visit the [official Elecrow repository](https://github.com/Elecrow-RD/CrowPanel-Advance-5-HMI-ESP32-S3-AI-Powered-IPS-Touch-Screen-800x480).

## Library Features

### RGB Display (`crow_panel.rs`)
Integrates the ESP32-S3's RGB LCD peripheral using `esp-idf-svc`. It implements the `DrawTarget` trait from the `embedded-graphics` crate, allowing you to use a rich ecosystem of graphics libraries.

- Supports fast clearing and filled rectangles.
- Optimized for the 800x480 resolution.

### GT911 Touch (`gt911.rs`)
A driver for the GT911 touch controller connected via I2C.
- Detects multiple touch points.
- Provides X, Y coordinates and touch size.

### STC8H1K28 Co-processor (`stc8h1k28.rs`)
Manages auxiliary hardware functions via I2C.
- **Backlight**: Adjustable brightness levels.
- **Buzzer**: Simple beep control.
- **Mute**: Unmute/Mute audio output.
- **Hardware Reset**: Manages the co-processor's state via a dedicated GPIO.

### BM8563 RTC (`bm8563.rs`)
Provides real-time clock functionality via I2C.
- Get and set date and time.
- Century bit support.

### Wireless Module Port (`wireless.rs`)
Configures the expansion port for external wireless modules (e.g., LoRa).
- Dedicated pin definitions for CrowPanel Advance V1.1.
- SPI initialization helper.
- **Note**: Requires physical DIP switch configuration (S1=OFF, S0=ON).

## Getting Started

### Prerequisites
- [Rust](https://www.rust-lang.org/tools/install) and the [ESP-IDF Rust toolchain](https://github.com/esp-rs/esp-idf-template).
- `esp-idf-svc` and its dependencies (requires LLVM/Clang for bindgen).

### Example Usage

```rust
use crow_panel_advance_5::crow_panel::init_lcd_panel;
use crow_panel_advance_5::crow_panel::RgbDisplay;
use embedded_graphics::prelude::*;
use embedded_graphics::pixelcolor::Rgb565;
use embedded_graphics::primitives::{Rectangle, PrimitiveStyleBuilder};

fn main() {
    // Initialize the LCD panel
    let panel_handle = init_lcd_panel();
    let mut display = RgbDisplay::new(panel_handle);

    // Use embedded-graphics to draw
    display.clear(Rgb565::BLACK).unwrap();
    
    Rectangle::new(Point::new(100, 100), Size::new(200, 100))
        .into_styled(
            PrimitiveStyleBuilder::new()
                .fill_color(Rgb565::RED)
                .build()
        )
        .draw(&mut display)
        .unwrap();
}
```

## License

This project is dual-licensed under either:

- **MIT License** ([LICENSE-MIT](LICENSE-MIT))
- **Apache License, Version 2.0** ([LICENSE-APACHE](LICENSE-APACHE))

at your option.

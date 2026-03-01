//! A Rust library for the CrowPanel Advance 5.0 HMI (ESP32-S3).
//!
//! This library provides low-level and high-level abstractions for interacting with the hardware
//! features of the CrowPanel Advance 5.0, including:
//! - **RGB LCD Display**: 800x480 resolution with `embedded-graphics` support.
//! - **GT911 Touch Controller**: Capacitive touch screen support via I2C.
//! - **STC8H1K28 Co-processor**: Control for backlight, buzzer, and audio muting.
//! - **BM8563 RTC**: Real-time clock for time and date management.
//! - **Wireless Module Port**: Configuration for the expansion port (LoRa/Wireless).

pub mod crow_panel;
pub mod gt911;
pub mod stc8h1k28;
pub mod bm8563;
pub mod wireless;

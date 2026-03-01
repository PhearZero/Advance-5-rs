//! Support for the Wireless Module port on CrowPanel Advance 5" HMI.
//!
//! This module provides pin definitions and helpers to initialize the SPI-based expansion port,
//! typically used for LoRa or other wireless modules.

use esp_idf_svc::hal::gpio::{InputPin, OutputPin};
use esp_idf_svc::hal::peripheral::Peripheral;
use esp_idf_svc::hal::spi::*;

/// Configuration for the Wireless Module port on CrowPanel Advance 5" HMI (V1.1).
/// 
/// Note: To use this port, the physical DIP switches must be set:
/// S1 = 0 (OFF), S0 = 1 (ON).
/// 
/// Selecting this mode multiplexes GPIO4, GPIO5, and GPIO6 to the module connector.
pub struct WirelessModulePins {
    pub reset: i32,
    pub dio1: i32,
    pub busy: i32,
    pub nss: i32,
    pub sclk: i32,
    pub miso: i32,
    pub mosi: i32,
}

/// Standard pinout for the Wireless Module port on CrowPanel V1.1.
pub const V1_1_WM_PINS: WirelessModulePins = WirelessModulePins {
    reset: 19,
    dio1: 20,
    busy: 2,
    nss: 8,
    sclk: 5,
    miso: 4,
    mosi: 6,
};

/// Helper to configure SPI for a LoRa/Wireless module on the CrowPanel V1.1 port.
pub fn init_spi_module_v1_1<SPI, SCK, MOSI, MISO>(
    spi: impl Peripheral<P = SPI> + 'static,
    sck: impl Peripheral<P = SCK> + 'static,
    mosi: impl Peripheral<P = MOSI> + 'static,
    miso: impl Peripheral<P = MISO> + 'static,
    baudrate_hz: u32,
) -> Result<SpiDriver<'static>, esp_idf_svc::sys::esp_err_t>
where
    SPI: SpiAnyPins,
    SCK: OutputPin,
    MOSI: OutputPin,
    MISO: InputPin + OutputPin,
{
    let config = config::Config::new().baudrate(esp_idf_svc::hal::units::Hertz(baudrate_hz));
    SpiDriver::new(
        spi,
        sck,
        mosi,
        Some(miso),
        &config::DriverConfig::new(),
    ).map_err(|e| e.code())
}

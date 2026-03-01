//! Driver for the STC8H1K28 co-processor.
//!
//! The STC8H1K28 microcontroller handles auxiliary hardware functions such as
//! backlight brightness, buzzer control, and audio muting via I2C.

use esp_idf_svc::hal::gpio::{InputPin, OutputPin, PinDriver, Pull};
use esp_idf_svc::hal::i2c::I2cDriver;
use esp_idf_svc::hal::peripheral::Peripheral;
use log::info;
use std::thread;
use std::time::Duration;

pub const STC8H1K28_ADDR: u8 = 0x30;

/// Possible commands to be sent to the STC8H1K28 co-processor.
pub enum Command {
    BacklightOff = 0x05,
    BacklightMin = 0x06,
    BacklightLow = 0x07,
    BacklightMed = 0x08,
    BacklightHigh = 0x09,
    BacklightMax = 0x10,
    BuzzerOn = 0x15,
    BuzzerOff = 0x16,
    Unmute = 0x17,
    Init = 0x18,
    PowerReset = 0x19,
}

/// Peripheral Controller for CrowPanel Advance 5" HMI (V1.1).
///
/// The STC8H1K28 microcontroller at I2C address 0x30 handles backlight, buzzer, and audio muting.
pub struct Stc8h1k28<'a, 'b> {
    i2c: &'a mut I2cDriver<'b>,
}

impl<'a, 'b> Stc8h1k28<'a, 'b> {
    /// Create a new STC8H1K28 driver instance.
    pub fn new(i2c: &'a mut I2cDriver<'b>) -> Self {
        Self { i2c }
    }

    /// Reset the STC8H1K28 microcontroller using the specified reset pin.
    /// This should be called BEFORE initializing the I2C driver on pins 15, 16.
    pub fn hardware_reset<P>(pin: P) -> Result<(), esp_idf_svc::sys::esp_err_t>
    where
        P: Peripheral + 'static,
        P::P: OutputPin + InputPin,
    {
        info!("Resetting STC8H1K28 microcontroller via GPIO1...");
        let mut stc_reset = PinDriver::output(pin).map_err(|e| e.code())?;
        stc_reset.set_low().map_err(|e| e.code())?;
        thread::sleep(Duration::from_millis(150));

        // Transition to high-impedance input mode to allow the chip to run
        let mut stc_input = stc_reset.into_input().map_err(|e| e.code())?;
        stc_input.set_pull(Pull::Floating).map_err(|e| e.code())?;

        thread::sleep(Duration::from_millis(100));
        Ok(())
    }

    /// Send a command to the co-processor.
    pub fn send_command(&mut self, cmd: Command) -> Result<(), esp_idf_svc::sys::esp_err_t> {
        self.i2c
            .write(STC8H1K28_ADDR, &[cmd as u8], 1000)
            .map_err(|e| e.code())
    }

    /// Initialize the co-processor.
    pub fn init(&mut self) -> Result<(), esp_idf_svc::sys::esp_err_t> {
        self.send_command(Command::Init).map(|_| {
            info!("STC8H1K28 initialized");
            thread::sleep(Duration::from_millis(10));
        })
    }

    /// Set the backlight brightness level (valid range 0x05 to 0x10).
    pub fn set_backlight(&mut self, brightness: u8) -> Result<(), esp_idf_svc::sys::esp_err_t> {
        // Clamping to valid range 0x05 to 0x10
        let val = brightness.clamp(0x05, 0x10);
        self.i2c
            .write(STC8H1K28_ADDR, &[val], 1000)
            .map_err(|e| e.code())
    }

    /// Turn the buzzer on.
    pub fn buzzer_on(&mut self) -> Result<(), esp_idf_svc::sys::esp_err_t> {
        self.send_command(Command::BuzzerOn)
    }

    /// Turn the buzzer off.
    pub fn buzzer_off(&mut self) -> Result<(), esp_idf_svc::sys::esp_err_t> {
        self.send_command(Command::BuzzerOff)
    }

    /// Beep the buzzer for a specified duration in milliseconds.
    pub fn beep(&mut self, duration_ms: u64) -> Result<(), esp_idf_svc::sys::esp_err_t> {
        self.buzzer_on()?;
        thread::sleep(Duration::from_millis(duration_ms));
        self.buzzer_off()?;
        Ok(())
    }

    /// Unmute the audio output.
    pub fn unmute(&mut self) -> Result<(), esp_idf_svc::sys::esp_err_t> {
        self.send_command(Command::Unmute)
    }
}

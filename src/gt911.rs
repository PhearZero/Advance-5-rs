//! Driver for the GT911 capacitive touch controller.
//!
//! The GT911 is connected via I2C and supports up to 5 simultaneous touch points.

use esp_idf_svc::hal::i2c::I2cDriver;

pub const GT911_ADDR: u8 = 0x5D;

const REG_POINT_INFO: u16 = 0x814E;
const REG_POINT_1: u16 = 0x814F;

/// Represents a single touch point.
#[derive(Debug, Clone, Copy, Default)]
pub struct TouchPoint {
    pub id: u8,
    pub x: u16,
    pub y: u16,
    pub size: u16,
}

/// GT911 touch driver instance.
pub struct Gt911<'a, 'b> {
    i2c: &'a mut I2cDriver<'b>,
    addr: u8,
}

impl<'a, 'b> Gt911<'a, 'b> {
    /// Create a new GT911 driver instance.
    pub fn new(i2c: &'a mut I2cDriver<'b>, addr: u8) -> Self {
        Self { i2c, addr }
    }

    fn write_reg(&mut self, reg: u16, val: u8) -> Result<(), esp_idf_svc::sys::esp_err_t> {
        let data = [(reg >> 8) as u8, (reg & 0xFF) as u8, val];
        self.i2c
            .write(self.addr, &data, 1000)
            .map_err(|e| e.code())?;
        Ok(())
    }

    fn read_regs(&mut self, reg: u16, buf: &mut [u8]) -> Result<(), esp_idf_svc::sys::esp_err_t> {
        let reg_bytes = [(reg >> 8) as u8, (reg & 0xFF) as u8];
        self.i2c
            .write_read(self.addr, &reg_bytes, buf, 1000)
            .map_err(|e| e.code())?;
        Ok(())
    }

    /// Read the touch status (whether any points are touched and how many).
    pub fn read_status(&mut self) -> Result<(bool, u8), esp_idf_svc::sys::esp_err_t> {
        let mut status = [0u8; 1];
        self.read_regs(REG_POINT_INFO, &mut status)?;
        let point_info = status[0];
        let buffer_status = (point_info >> 7) & 1;
        let touches = point_info & 0x0F;
        Ok((buffer_status == 1, touches))
    }

    /// Clear the touch status buffer.
    pub fn clear_status(&mut self) -> Result<(), esp_idf_svc::sys::esp_err_t> {
        self.write_reg(REG_POINT_INFO, 0)
    }

    /// Access the underlying I2C driver.
    pub fn i2c_mut(&mut self) -> &mut I2cDriver<'b> {
        self.i2c
    }

    /// Read multiple touch points.
    pub fn read_points(
        &mut self,
        num_touches: u8,
    ) -> Result<Vec<TouchPoint>, esp_idf_svc::sys::esp_err_t> {
        let mut points = Vec::with_capacity(num_touches as usize);
        for i in 0..num_touches {
            let mut data = [0u8; 7];
            self.read_regs(REG_POINT_1 + (i as u16 * 8), &mut data)?;

            let id = data[0];
            let x = u16::from_le_bytes([data[1], data[2]]);
            let y = u16::from_le_bytes([data[3], data[4]]);
            let size = u16::from_le_bytes([data[5], data[6]]);

            points.push(TouchPoint { id, x, y, size });
        }
        self.clear_status()?;
        Ok(points)
    }
}

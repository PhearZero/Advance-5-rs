//! Driver for the RGB LCD display on CrowPanel Advance 5" HMI.
//!
//! This module provides a `DrawTarget` implementation for the ESP32-S3's RGB LCD peripheral,
//! allowing it to be used with the `embedded-graphics` crate.

use embedded_graphics::Pixel;
use embedded_graphics::draw_target::DrawTarget;
use embedded_graphics::geometry::{OriginDimensions, Size};
use embedded_graphics::pixelcolor::Rgb565;
use embedded_graphics::pixelcolor::raw::{RawData, RawU16};
use esp_idf_svc::sys::*;

pub const DISPLAY_SIZE: (u16, u16) = (800, 480);

#[repr(C)]
pub struct esp_lcd_rgb_panel_config_t {
    pub clk_src: lcd_clock_source_t,
    pub timings: esp_lcd_rgb_timing_t,
    pub data_width: usize,
    pub bits_per_pixel: usize,
    pub num_fbs: usize,
    pub bounce_buffer_size_px: usize,
    pub sram_trans_align: usize,
    pub psram_trans_align: usize, // union { psram_trans_align | dma_burst_size } – keep layout-compatible size_t
    pub hsync_gpio_num: i32,
    pub vsync_gpio_num: i32,
    pub de_gpio_num: i32,
    pub pclk_gpio_num: i32,
    pub disp_gpio_num: i32,
    pub data_gpio_nums: [i32; 16],
    pub flags: esp_lcd_rgb_panel_config_t__bindgen_ty_1,
}

#[repr(C)]
pub struct esp_lcd_rgb_panel_config_t__bindgen_ty_1 {
    pub _bitfield_1: u32,
}

#[repr(C)]
pub struct esp_lcd_rgb_timing_t {
    pub pclk_hz: u32,
    pub h_res: u32,
    pub v_res: u32,
    pub hsync_pulse_width: u32,
    pub hsync_back_porch: u32,
    pub hsync_front_porch: u32,
    pub vsync_pulse_width: u32,
    pub vsync_back_porch: u32,
    pub vsync_front_porch: u32,
    pub flags: esp_lcd_rgb_timing_t__bindgen_ty_1,
}

#[repr(C)]
pub struct esp_lcd_rgb_timing_t__bindgen_ty_1 {
    pub _bitfield_1: u32,
}

unsafe extern "C" {
    pub fn esp_lcd_new_rgb_panel(
        config: *const esp_lcd_rgb_panel_config_t,
        ret_panel: *mut esp_lcd_panel_handle_t,
    ) -> esp_err_t;
}

/// RGB Display driver.
pub struct RgbDisplay {
    panel: esp_lcd_panel_handle_t,
}

impl RgbDisplay {
    /// Create a new RgbDisplay instance from a panel handle.
    pub fn new(panel: esp_lcd_panel_handle_t) -> Self {
        Self { panel }
    }
}

impl DrawTarget for RgbDisplay {
    type Color = Rgb565;
    type Error = core::convert::Infallible;

    fn draw_iter<I>(&mut self, pixels: I) -> Result<(), Self::Error>
    where
        I: IntoIterator<Item = Pixel<Self::Color>>,
    {
        for Pixel(point, color) in pixels {
            if point.x >= 0
                && point.x < DISPLAY_SIZE.0 as i32
                && point.y >= 0
                && point.y < DISPLAY_SIZE.1 as i32
            {
                let raw_val = RawU16::from(color).into_inner();
                unsafe {
                    esp_lcd_panel_draw_bitmap(
                        self.panel,
                        point.x,
                        point.y,
                        point.x + 1,
                        point.y + 1,
                        &raw_val as *const u16 as *const _,
                    );
                }
            }
        }
        Ok(())
    }

    fn fill_contiguous<I>(
        &mut self,
        area: &embedded_graphics::primitives::Rectangle,
        colors: I,
    ) -> Result<(), Self::Error>
    where
        I: IntoIterator<Item = Self::Color>,
    {
        let mut colors_iter = colors.into_iter();
        let x_start = area.top_left.x.max(0) as u16;
        let y_start = area.top_left.y.max(0) as u16;
        let width = area.size.width as u16;
        let height = area.size.height as u16;

        let num_pixels = (width as usize) * (height as usize);
        let mut buffer = Vec::with_capacity(num_pixels);
        for _ in 0..num_pixels {
            if let Some(color) = colors_iter.next() {
                let raw_val = RawU16::from(color).into_inner();
                buffer.push(raw_val);
            }
        }

        unsafe {
            esp_lcd_panel_draw_bitmap(
                self.panel,
                x_start as i32,
                y_start as i32,
                (x_start + width) as i32,
                (y_start + height) as i32,
                buffer.as_ptr() as *const _,
            );
        }
        Ok(())
    }

    fn clear(&mut self, color: Self::Color) -> Result<(), Self::Error> {
        let num_pixels = DISPLAY_SIZE.0 as usize * DISPLAY_SIZE.1 as usize;
        let raw_val = RawU16::from(color).into_inner();
        let buffer = vec![raw_val; num_pixels];
        unsafe {
            esp_lcd_panel_draw_bitmap(
                self.panel,
                0,
                0,
                DISPLAY_SIZE.0 as i32,
                DISPLAY_SIZE.1 as i32,
                buffer.as_ptr() as *const _,
            );
        }
        Ok(())
    }
}

impl OriginDimensions for RgbDisplay {
    fn size(&self) -> Size {
        Size::new(DISPLAY_SIZE.0 as u32, DISPLAY_SIZE.1 as u32)
    }
}

/// Initialize the LCD panel with default configuration for the CrowPanel Advance 5.0.
///
/// Returns a handle to the initialized LCD panel.
pub fn init_lcd_panel() -> esp_lcd_panel_handle_t {
    let timing = esp_lcd_rgb_timing_t {
        pclk_hz: 16_000_000,
        h_res: 800,
        v_res: 480,
        hsync_pulse_width: 8,
        hsync_back_porch: 20,
        hsync_front_porch: 20,
        vsync_pulse_width: 8,
        vsync_back_porch: 20,
        vsync_front_porch: 20,
        flags: esp_lcd_rgb_timing_t__bindgen_ty_1 {
            // flags bitfield layout (see esp_lcd_rgb_timing_t in IDF):
            // bit0: hsync_idle_low, bit1: vsync_idle_low, bit2: de_idle_high,
            // bit3: pclk_active_neg, bit4: pclk_idle_high
            _bitfield_1: (1 << 3), // pclk_active_neg
        },
    };

    let config = esp_lcd_rgb_panel_config_t {
        clk_src: soc_periph_lcd_clk_src_t_LCD_CLK_SRC_PLL160M,
        timings: timing,
        data_width: 16,
        bits_per_pixel: 16,
        num_fbs: 2,
        bounce_buffer_size_px: 800 * 10,
        sram_trans_align: 8,
        psram_trans_align: 64,
        hsync_gpio_num: 40,
        vsync_gpio_num: 41,
        de_gpio_num: 42,
        pclk_gpio_num: 39,
        disp_gpio_num: -1,
        data_gpio_nums: [
            21, 47, 48, 45, 38, // Blue
            9, 10, 11, 12, 13, 14, // Green
            7, 17, 18, 3, 46, // Red
        ],
        flags: esp_lcd_rgb_panel_config_t__bindgen_ty_1 {
            _bitfield_1: (1 << 2) | (1 << 3), // fb_in_psram (bit 2) | double_fb (bit 3)
        },
    };

    let mut panel: esp_lcd_panel_handle_t = std::ptr::null_mut();
    unsafe {
        let err = esp_lcd_new_rgb_panel(&config, &mut panel);
        if err != 0 {
            panic!("Failed to create RGB panel: error code {}", err);
        }
        esp_lcd_panel_reset(panel);
        esp_lcd_panel_init(panel);
    }
    panel
}

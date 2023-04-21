use noto_sans_mono_bitmap::{RasterizedChar, RasterHeight, FontWeight, get_raster_width, get_raster};
use bootloader_api::info::{FrameBufferInfo, PixelFormat};
use core::fmt;

use crate::math::Vector2;


pub const CHARACTER_HEIGHT: RasterHeight = RasterHeight::Size16;
pub const CHARACTER_WIDTH: usize = get_raster_width(
    FontWeight::Regular, CHARACTER_HEIGHT
);

pub const LETTER_SPACING: usize = 0;
pub const LINE_SPACING: usize = 2;
pub const PADDING: usize = 2;


pub struct FontWriter<'a> {
    frame_buffer_info: FrameBufferInfo,
    frame_buffer: &'a mut [u8],
    position: Vector2<usize>,
}


impl<'a> FontWriter<'a> {
    pub fn new(frame_buffer_info: FrameBufferInfo, frame_buffer: &'a mut [u8]) -> Self {
        Self {
            position: Vector2 { x: 0, y: 0 },
            frame_buffer_info,
            frame_buffer,
        }
    }

    pub fn clear(&mut self) {
        self.position = Vector2 { x: PADDING, y: PADDING };
        self.frame_buffer.fill(0);
    }

    fn write_character(&mut self, character: char) {
        match character {
            '\r' => {
                self.position.x = PADDING;
            },

            '\n' => {
                self.position.y += CHARACTER_HEIGHT.val() + LINE_SPACING;
                self.write_character('\r');
            },

            character => {
                let new_x = self.position.x + CHARACTER_WIDTH;

                if new_x >= self.frame_buffer_info.width {
                    self.write_character('\n');
                }

                let new_y = self.position.y + CHARACTER_HEIGHT.val() + PADDING;

                if new_y >= self.frame_buffer_info.height {
                    self.clear();
                }

                let character = get_raster(character, FontWeight::Regular, CHARACTER_HEIGHT)
                    .expect("");
                self.write_rasterized(character);
            }
        }
    }

    fn write_rasterized(&mut self, character: RasterizedChar) {
        for (i, row) in character.raster().iter().enumerate() {
            for (j, byte) in row.iter().enumerate() {
                let position = Vector2 {
                    x: self.position.x + j,
                    y: self.position.y + i,
                };

                self.write_pixel(position, *byte);
            }
        }

        self.position.x += character.width() + LETTER_SPACING;
    }

    fn write_pixel(&mut self, position: Vector2<usize>, intensity: u8) {
        let offset = position.y * self.frame_buffer_info.stride + position.x;

        let pixel_color = match self.frame_buffer_info.pixel_format {
            PixelFormat::Rgb => [ intensity, intensity, intensity / 2, 0 ],
            PixelFormat::Bgr => [ intensity / 2, intensity, intensity, 0 ],
            PixelFormat::U8 => [ 0, 0, 0, 0 ],
            _ => core::unreachable!(),
        };

        let byte_offset = offset * self.frame_buffer_info.bytes_per_pixel;

        self.frame_buffer[byte_offset .. (byte_offset + self.frame_buffer_info.bytes_per_pixel)]
            .copy_from_slice(&pixel_color[..self.frame_buffer_info.bytes_per_pixel]);

        let _ = unsafe {
            core::ptr::read_volatile(&self.frame_buffer[byte_offset])
        };
    }
}


impl<'a> fmt::Write for FontWriter<'a> {
    fn write_str(&mut self, string: &str) -> fmt::Result {
        for character in string.chars() {
            self.write_character(character);
        }

        Ok(())
    }
}
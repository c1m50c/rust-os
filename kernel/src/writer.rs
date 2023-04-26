use noto_sans_mono_bitmap::{RasterizedChar, RasterHeight, FontWeight, get_raster_width, get_raster};
use bootloader_api::info::{FrameBufferInfo, PixelFormat};
use fixed_vectors::{Vector2, Vector3};
use core::fmt;


pub const CHARACTER_HEIGHT: RasterHeight = RasterHeight::Size16;
pub const CHARACTER_WIDTH: usize = get_raster_width(
    FontWeight::Regular, CHARACTER_HEIGHT
);

pub const LETTER_SPACING: usize = 0;
pub const LINE_SPACING: usize = 2;
pub const PADDING: usize = 2;


pub struct FrameBufferWriter<'a> {
    pub frame_buffer_info: FrameBufferInfo,
    frame_buffer: &'a mut [u8],
    position: Vector2<usize>,
}


impl<'a> FrameBufferWriter<'a> {
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
                self.write_rasterized_character(character);
            }
        }
    }

    fn write_rasterized_character(&mut self, character: RasterizedChar) {
        for (i, row) in character.raster().iter().enumerate() {
            for (j, byte) in row.iter().enumerate() {
                let position = Vector2 {
                    x: self.position.x + j,
                    y: self.position.y + i,
                };

                self.write_pixel(position, Vector3::new(*byte, 0, 0));
            }
        }

        self.position.x += character.width() + LETTER_SPACING;
    }

    pub fn write_pixel(&mut self, position: Vector2<usize>, color: Vector3<u8>) {
        let offset = position.y * self.frame_buffer_info.stride + position.x;

        let pixel_color = match self.frame_buffer_info.pixel_format {
            PixelFormat::Rgb | PixelFormat::U8 => [ color.x, color.y, color.z, 0 ],
            PixelFormat::Bgr => [ color.z, color.y, color.x, 0 ], // Upside Down
            _ => core::unreachable!(),
        };

        let byte_offset = offset * self.frame_buffer_info.bytes_per_pixel;

        self.frame_buffer[byte_offset .. (byte_offset + self.frame_buffer_info.bytes_per_pixel)]
            .copy_from_slice(&pixel_color[..self.frame_buffer_info.bytes_per_pixel]);

        let _ = unsafe {
            // SAFETY: `self.frame_buffer[..]` is guranteed to meet the safety contract of `read_volatile`
            core::ptr::read_volatile(&self.frame_buffer[byte_offset])
        };
    }
}


impl<'a> fmt::Write for FrameBufferWriter<'a> {
    fn write_str(&mut self, string: &str) -> fmt::Result {
        for character in string.chars() {
            self.write_character(character);
        }

        Ok(())
    }
}
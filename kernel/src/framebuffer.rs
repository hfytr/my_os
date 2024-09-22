use bootloader_api::info::{self, FrameBufferInfo, Optional};
use bytemuck::{from_bytes, from_bytes_mut, Pod, Zeroable};
use core::ops::{Index, IndexMut};

use crate::font::{FONT, FONT_HEIGHT, FONT_WIDTH};

const BLACK: Pixel = Pixel {
    b: 0x00,
    g: 0x00,
    r: 0x00,
    alpha: 0x00,
};

const WHITE: Pixel = Pixel {
    b: 0xff,
    g: 0xff,
    r: 0xff,
    alpha: 0x00,
};

#[derive(Zeroable, Pod, Clone, Copy)]
#[repr(C)]
pub struct Pixel {
    pub b: u8,
    pub g: u8,
    pub r: u8,
    pub alpha: u8,
}

pub struct FrameBuffer<'a> {
    pub width: usize,
    pub height: usize,
    pub stride: usize,
    pub bbp: usize,
    buffer: &'a mut [u8],
}

impl<'a> FrameBuffer<'a> {
    pub fn new(framebuffer: &'a mut Optional<info::FrameBuffer>) -> FrameBuffer<'a> {
        let framebuffer = framebuffer.as_mut().unwrap();
        let info = framebuffer.info();
        let buffer = framebuffer.buffer_mut();
        let FrameBufferInfo {
            width,
            height,
            stride,
            bytes_per_pixel: bbp,
            ..
        } = info;
        Self {
            width,
            height,
            stride,
            bbp,
            buffer,
        }
    }

    pub fn write_str(&mut self, bytes: &[u8]) {
        for (byte_num, byte) in bytes.iter().enumerate() {
            for i in 0..FONT_WIDTH as usize {
                for j in 0..FONT_HEIGHT as usize {
                    self[(byte_num * FONT_WIDTH as usize + i, j)] =
                        if FONT[*byte as usize] & 1 << (j * FONT_WIDTH as usize + i) > 0 {
                            WHITE
                        } else {
                            BLACK
                        }
                }
            }
        }
    }
}

impl<'a> Index<(usize, usize)> for FrameBuffer<'a> {
    type Output = Pixel;
    fn index(&self, (x, y): (usize, usize)) -> &Pixel {
        let pixel_index = (y * self.stride + x) * self.bbp;
        from_bytes(&self.buffer[pixel_index..pixel_index + 4])
    }
}

impl<'a> IndexMut<(usize, usize)> for FrameBuffer<'a> {
    fn index_mut(&mut self, (x, y): (usize, usize)) -> &mut Pixel {
        let pixel_index = (y * self.stride + x) * self.bbp;
        from_bytes_mut(&mut self.buffer[pixel_index..pixel_index + 4])
    }
}

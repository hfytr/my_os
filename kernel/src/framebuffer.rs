use bootloader_api::info::{self, FrameBufferInfo, Optional};
use bytemuck::{from_bytes, from_bytes_mut, Pod, Zeroable};
use core::ops::{Index, IndexMut};

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

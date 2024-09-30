use bootloader_api::info::{self, FrameBufferInfo, Optional};
use bytemuck::{from_bytes, from_bytes_mut, Pod, Zeroable};
use core::fmt::{self, Arguments, Write};
use core::ops::{Index, IndexMut};
use spin::Mutex;
use x86_64::instructions::interrupts;

use crate::font::{FONT, FONT_DIM};

pub static FRAMEBUFFER: Mutex<FrameBuffer> = Mutex::new(FrameBuffer::const_default());

const BLACK: Pixel = Pixel {
    b: 0x00,
    g: 0x00,
    r: 0x00,
    alpha: 0x00,
};

const GREEN: Pixel = Pixel {
    b: 0x00,
    g: 0xff,
    r: 0x00,
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

#[macro_export]
macro_rules! print {
    ($($arg:tt)*) => ($crate::framebuffer::_print(format_args!($($arg)*)));
}

#[macro_export]
macro_rules! println {
    () => ($crate::print!("\n"));
    ($($arg:tt)*) => ($crate::print!("{}\n", format_args!($($arg)*)));
}

#[doc(hidden)]
pub fn _print(args: Arguments) {
    interrupts::without_interrupts(|| {
        FRAMEBUFFER
            .lock()
            .write_fmt(args)
            .expect("failed to write FRAMEBUFFER")
    });
}

pub struct FrameBuffer {
    pixel_dim: (usize, usize),
    pub term_dim: (usize, usize),
    stride: usize,
    bbp: usize,
    pos: (usize, usize),
    buffer: Option<&'static mut [u8]>,
    lines: [[u8; 256]; 64],
    last_line: usize,
}

impl FrameBuffer {
    pub fn new(framebuffer: &'static mut Optional<info::FrameBuffer>) -> FrameBuffer {
        let framebuffer = framebuffer.as_mut().expect("failed to write FRAMEBUFFER");
        let info = framebuffer.info();
        let buffer = Some(framebuffer.buffer_mut());
        let FrameBufferInfo {
            width,
            height,
            stride,
            bytes_per_pixel: bbp,
            ..
        } = info;

        let mut framebuffer = Self {
            pixel_dim: (width, height),
            term_dim: (width / FONT_DIM.0 as usize, height / FONT_DIM.1 as usize),
            stride,
            bbp,
            pos: (0, 0),
            buffer,
            lines: [[b' '; 256]; 64],
            last_line: 0,
        };

        for x in 0..framebuffer.pixel_dim.0 {
            for y in 0..framebuffer.pixel_dim.1 {
                framebuffer[(x, y)] = BLACK;
            }
        }

        framebuffer
    }

    pub const fn const_default() -> FrameBuffer {
        FrameBuffer {
            pixel_dim: (0, 0),
            term_dim: (0, 0),
            stride: 0,
            bbp: 0,
            pos: (0, 0),
            buffer: None,
            lines: [[b' '; 256]; 64],
            last_line: 0,
        }
    }

    pub fn write_str(&mut self, bytes: &[u8]) {
        for &byte in bytes {
            match byte {
                b'\n' => {
                    if self.pos.1 < self.term_dim.1 - 1 {
                        for i in self.pos.0..self.term_dim.0 {
                            self.lines[self.last_line as usize][i as usize] = b' ';
                        }
                        self.pos.1 += 1;
                        self.pos.0 = 0;
                        self.last_line = (self.last_line + 1) % self.term_dim.1;
                    } else {
                        self.last_line = (self.last_line + 1) % self.term_dim.1;
                        for i in 0..self.lines[0].len() {
                            self.lines[self.last_line + 2][i] = b' ';
                        }
                        for (y, line) in (0..self.lines.len())
                            .cycle()
                            .skip(self.last_line - 1)
                            .take(self.term_dim.1)
                            .enumerate()
                        {
                            for (x, c) in self.lines[line].into_iter().enumerate() {
                                self.render_char(c, Some((x, y)));
                            }
                        }
                    }
                }
                _ => {
                    self.render_char(byte, None);
                    self.lines[self.pos.1][self.pos.0] = byte;
                    self.pos.0 = (self.pos.0 + 1).min(self.term_dim.0 - 1);
                    if self.pos.0 == self.term_dim.0 - 1 {
                        self.pos.0 = 0;
                        self.pos.1 += 1;
                    }
                }
            }
        }
    }

    fn render_char(&mut self, byte: u8, pos: Option<(usize, usize)>) {
        let (x, y) = pos.unwrap_or(self.pos);
        for i in 0..FONT_DIM.0 as usize {
            for j in 0..FONT_DIM.1 as usize {
                let index = (x * FONT_DIM.0 as usize + i, y * FONT_DIM.1 as usize + j);
                self[index] = if FONT[byte as usize] & 1 << (j * FONT_DIM.0 as usize + i) > 0 {
                    GREEN
                } else {
                    BLACK
                }
            }
        }
    }
}

impl Write for FrameBuffer {
    fn write_str(&mut self, s: &str) -> fmt::Result {
        self.write_str(s.as_bytes());
        Ok(())
    }
}

impl Index<(usize, usize)> for FrameBuffer {
    type Output = Pixel;
    fn index(&self, (x, y): (usize, usize)) -> &Pixel {
        let pixel_index = (y * self.stride + x) * self.bbp;
        from_bytes(&self.buffer.as_ref().unwrap()[pixel_index..pixel_index + 4])
    }
}

impl IndexMut<(usize, usize)> for FrameBuffer {
    fn index_mut(&mut self, (x, y): (usize, usize)) -> &mut Pixel {
        let pixel_index = (y * self.stride + x) * self.bbp;
        from_bytes_mut(&mut self.buffer.as_mut().unwrap()[pixel_index..pixel_index + 4])
    }
}

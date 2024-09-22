#![no_std]
#![no_main]

mod font;
mod framebuffer;

use bootloader_api::{entry_point, info::BootInfo};
use core::fmt::Write;
use core::panic::PanicInfo;
use framebuffer::FrameBuffer;

entry_point!(kernel_main);
fn kernel_main(boot_info: &'static mut BootInfo) -> ! {
    let mut framebuffer = FrameBuffer::new(&mut boot_info.framebuffer);
    write!(framebuffer, "hello: {}", 11).unwrap();
    loop {}
}

#[panic_handler]
fn panic(_info: &PanicInfo) -> ! {
    loop {}
}

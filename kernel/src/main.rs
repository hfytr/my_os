#![no_std]
#![no_main]

mod font;
mod framebuffer;

use bootloader_api::{entry_point, info::BootInfo};
use core::panic::PanicInfo;
use framebuffer::{FrameBuffer, FRAMEBUFFER};

entry_point!(kernel_main);
fn kernel_main(boot_info: &'static mut BootInfo) -> ! {
    *FRAMEBUFFER.lock() = FrameBuffer::new(&mut boot_info.framebuffer);
    println!("HELLO WORLD!");
    loop {}
}

#[panic_handler]
fn panic(info: &PanicInfo) -> ! {
    println!("{}", info);
    loop {}
}

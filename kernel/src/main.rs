#![no_std]
#![no_main]
#![feature(abi_x86_interrupt)]

mod font;
mod framebuffer;
mod interrupt;

use bootloader_api::{entry_point, info::BootInfo};
use core::panic::PanicInfo;
use framebuffer::{FrameBuffer, FRAMEBUFFER};
use interrupt::init_idt;

entry_point!(kernel_main);
fn kernel_main(boot_info: &'static mut BootInfo) -> ! {
    *FRAMEBUFFER.lock() = FrameBuffer::new(&mut boot_info.framebuffer);
    init_idt();
    x86_64::instructions::interrupts::int3();
    println!("HELLO WORLD!");
    loop {}
}

#[panic_handler]
fn panic(info: &PanicInfo) -> ! {
    println!("{}", info);
    loop {}
}

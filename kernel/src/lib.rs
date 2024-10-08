#![no_std]
#![cfg_attr(test, no_main)]
#![feature(custom_test_frameworks)]
#![feature(abi_x86_interrupt)]
#![test_runner(crate::test_runner)]
#![reexport_test_harness_main = "test_main"]

mod font;
pub mod framebuffer;
mod gdt;
pub mod interrupt;
mod serial;

use bootloader_api::{info::BootInfo, BootloaderConfig};
use core::panic::PanicInfo;
use framebuffer::{FrameBuffer, BLACK, FRAMEBUFFER};
use gdt::init_gdt;
use interrupt::init_idt;
use serial::SERIAL1;
use x86_64::instructions::{self, port::Port};

pub const fn bootloader_config() -> BootloaderConfig {
    let mut config = BootloaderConfig::new_default();
    config.mappings.dynamic_range_start = Some(0xffff_8000_0000_0000);
    config.mappings.dynamic_range_end = Some(0xffff_ffff_ffff_ffff);
    config
}

pub fn init(boot_info: &'static mut BootInfo) {
    *FRAMEBUFFER.lock() = FrameBuffer::new(&mut boot_info.framebuffer);
    FRAMEBUFFER.lock().fill(BLACK);
    SERIAL1.lock().init();
    init_gdt();
    init_idt();
}

pub fn test_runner(tests: &[&dyn Testable]) {
    serial_println!("Running {} tests", tests.len());
    for test in tests {
        test.run();
    }
}

pub fn panic_test(info: &PanicInfo) -> ! {
    serial_println!("[failed]");
    serial_println!("info: {}", info);
    exit_qemu(QemuExitCode::Failed);
    loop {
        instructions::hlt();
    }
}

#[cfg(test)]
#[panic_handler]
fn panic(info: &PanicInfo) -> ! {
    panic_test(info);
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
#[repr(u32)]
pub enum QemuExitCode {
    Success = 0x10,
    Failed = 0x11,
}

pub fn exit_qemu(exit_code: QemuExitCode) {
    unsafe {
        let mut port = Port::new(0xf4);
        port.write(exit_code as u32);
    }
}

pub trait Testable {
    fn run(&self);
}

impl<T: Fn()> Testable for T {
    fn run(&self) {
        serial_println!("running test {}...", core::any::type_name::<T>());
        self();
        serial_println!("[ok]");
    }
}

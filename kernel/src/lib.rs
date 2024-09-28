#![no_std]
#![cfg_attr(test, no_main)]
#![feature(custom_test_frameworks)]
#![feature(abi_x86_interrupt)]
#![test_runner(crate::test_runner)]
#![reexport_test_harness_main = "test_main"]

mod font;
pub mod framebuffer;
mod gdt;
mod interrupt;
mod serial;

use bootloader_api::{entry_point, info::BootInfo};
use core::panic::PanicInfo;
use framebuffer::{FrameBuffer, FRAMEBUFFER};
use gdt::init_gdt;
use interrupt::init_idt;
use serial::SERIAL1;
use x86_64::instructions::port::Port;

pub fn init(boot_info: &'static mut BootInfo) {
    *FRAMEBUFFER.lock() = FrameBuffer::new(&mut boot_info.framebuffer);
    SERIAL1.lock().init();
    init_idt();
    init_gdt();
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
    loop {}
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

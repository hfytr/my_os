#![no_std]
#![no_main]
#![feature(custom_test_frameworks)]
#![test_runner(kernel::test_runner)]
#![reexport_test_harness_main = "test_main"]
#![feature(abi_x86_interrupt)]

use bootloader_api::{entry_point, info::BootInfo};
use core::panic::PanicInfo;
use kernel::{init, println};

#[cfg(not(test))]
entry_point!(kernel_main);
fn kernel_main(boot_info: &'static mut BootInfo) -> ! {
    init(boot_info);
    println!("HELLO WORLD!");

    #[cfg(test)]
    test_main();

    loop {}
}

#[cfg(not(test))]
#[panic_handler]
fn panic(info: &PanicInfo) -> ! {
    println!("{}", info);
    loop {}
}

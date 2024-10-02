#![no_std]
#![no_main]
#![feature(custom_test_frameworks)]
#![test_runner(kernel::test_runner)]
#![reexport_test_harness_main = "test_main"]
#![feature(abi_x86_interrupt)]

use bootloader_api::{entry_point, info::BootInfo, BootloaderConfig};
use core::panic::PanicInfo;
use kernel::{
    bootloader_config, init,
    interrupt::{pop_call_stack, push_call_stack, CALL_STACK},
    print, println,
};
use x86_64::instructions;

const CONFIG: BootloaderConfig = bootloader_config();

entry_point!(kernel_main, config = &CONFIG);

fn kernel_main(boot_info: &'static mut BootInfo) -> ! {
    init(boot_info);

    #[cfg(test)]
    test_main();

    loop {
        instructions::hlt()
    }
}

#[panic_handler]
fn panic(info: &PanicInfo) -> ! {
    println!("{}", info);
    println!("CALL_STACK({}):", CALL_STACK.lock().0);
    let call_stack = CALL_STACK.lock();
    for i in 0..call_stack.0 {
        println!("    {}", call_stack.1[i as usize]);
    }
    loop {
        instructions::hlt();
    }
}

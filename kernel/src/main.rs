#![no_std]
#![no_main]
#![feature(custom_test_frameworks)]
#![test_runner(kernel::test_runner)]
#![reexport_test_harness_main = "test_main"]
#![feature(abi_x86_interrupt)]

use bootloader_api::{entry_point, info::BootInfo};
use core::panic::PanicInfo;
use kernel::{init, println};
use x86_64::instructions;

#[cfg(not(test))]
entry_point!(kernel_main);
fn kernel_main(boot_info: &'static mut BootInfo) -> ! {
    init(boot_info);

    #[cfg(test)]
    test_main();

    loop {
        instructions::hlt();
    }
}

#[cfg(not(test))]
#[panic_handler]
fn panic(info: &PanicInfo) -> ! {
    println!("{}", info);
    loop {
        instructions::hlt();
    }
}

/*
InterruptStackFrame {
    instruction_pointer: VirtAddr(0x8000009a7f),
    code_segment: SegmentSelector {
        index: 1,
        rpl: Ring0,
    },
    cpu_flags: RFlags(PARITY_FLAG | 0x2),
    stack_pointer: VirtAddr(0x10000014f98),
    stack_segment: SegmentSelector {
        index: 2,
        rpl: Ring0,
    },
}
*/

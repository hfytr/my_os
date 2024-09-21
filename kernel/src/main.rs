#![no_std]
#![no_main]

mod framebuffer;

use bootloader_api::{entry_point, info::BootInfo};
use core::panic::PanicInfo;
use framebuffer::{FrameBuffer, Pixel};

entry_point!(kernel_main);
fn kernel_main(boot_info: &'static mut BootInfo) -> ! {
    let mut framebuffer = FrameBuffer::new(&mut boot_info.framebuffer);

    for x in 0..framebuffer.width {
        for y in 0..framebuffer.height {
            framebuffer[(x, y)] = Pixel {
                b: 0xff,
                g: 0x00,
                r: 0xff,
                alpha: 0x00,
            };
        }
    }

    loop {}
}

#[panic_handler]
fn panic(_info: &PanicInfo) -> ! {
    loop {}
}

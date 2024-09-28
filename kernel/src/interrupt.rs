use crate::gdt::DOUBLE_FAULT_1ST_INDEX;
use crate::{print, println};
use pic8259::ChainedPics;
use spin::{Mutex, Once};
use x86_64::instructions::interrupts;
use x86_64::structures::idt::{InterruptDescriptorTable, InterruptStackFrame};

const PIC1_OFFSET: u8 = 32;
const PIC2_OFFSET: u8 = PIC1_OFFSET + 8;
static IDT: Once<InterruptDescriptorTable> = Once::new();
static PICS: Mutex<ChainedPics> = Mutex::new(unsafe { ChainedPics::new(PIC1_OFFSET, PIC2_OFFSET) });

#[derive(Debug, Clone, Copy)]
#[repr(u8)]
pub enum InterruptIndex {
    Timer = PIC1_OFFSET,
}

pub fn init_idt() {
    IDT.call_once(|| {
        let mut idt = InterruptDescriptorTable::new();
        idt.breakpoint.set_handler_fn(breakpoint_handler);
        unsafe {
            idt.double_fault
                .set_handler_fn(double_handler)
                .set_stack_index(DOUBLE_FAULT_1ST_INDEX);
        }
        idt
    });
    IDT.get().expect("failed to get IDT").load();
    unsafe {
        PICS.lock().initialize();
    }
    interrupts::enable();
}

extern "x86-interrupt" fn breakpoint_handler(stack_frame: InterruptStackFrame) {
    println!("EXCEPTION: BREAKPOINT\n{:#?}", stack_frame);
}

extern "x86-interrupt" fn double_handler(stack_frame: InterruptStackFrame, _error_code: u64) -> ! {
    println!("EXCEPTION: DOUBLE FAULT\n{:#?}", stack_frame);
    loop {}
}

extern "x86-interrupt" fn timer_handler(_stack_frame: InterruptStackFrame) {
    print!(".");
    unsafe {
        PICS.lock()
            .notify_end_of_interrupt(InterruptIndex::Timer as u8);
    }
}

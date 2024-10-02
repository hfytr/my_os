use crate::gdt::DOUBLE_FAULT_1ST_INDEX;
use crate::{print, println};
use pic8259::ChainedPics;
use spin::{Mutex, Once};
use x86_64::instructions::interrupts;
use x86_64::structures::idt::{InterruptDescriptorTable, InterruptStackFrame};

const PIC1_OFFSET: u8 = 32;
const PIC2_OFFSET: u8 = PIC1_OFFSET + 8;
pub static CALL_STACK: Mutex<(u8, [u8; 256])> = Mutex::new((0, [0; 256]));
static IDT: Once<InterruptDescriptorTable> = Once::new();
static PICS: Mutex<ChainedPics> = Mutex::new(unsafe { ChainedPics::new(PIC1_OFFSET, PIC2_OFFSET) });

pub fn push_call_stack(fn_name: u8) {
    let mut call_stack = CALL_STACK.lock();
    let stack_size = call_stack.0;
    call_stack.1[stack_size as usize] = fn_name;
    call_stack.0 += 1;
}

pub fn pop_call_stack() {
    CALL_STACK.lock().0 -= 1;
}

#[derive(Debug, Clone, Copy)]
#[repr(u8)]
pub enum InterruptIndex {
    Timer = PIC1_OFFSET,
    Keyboard,
}

pub fn init_idt() {
    push_call_stack(1);
    IDT.call_once(|| {
        let mut idt = InterruptDescriptorTable::new();
        idt.breakpoint.set_handler_fn(breakpoint_handler);
        unsafe {
            idt.double_fault
                .set_handler_fn(double_handler)
                .set_stack_index(DOUBLE_FAULT_1ST_INDEX);
        }
        idt.general_protection_fault.set_handler_fn(general_handler);
        idt[InterruptIndex::Timer as u8].set_handler_fn(timer_handler);
        idt[InterruptIndex::Keyboard as u8].set_handler_fn(keyboard_interrupt);
        idt
    });
    IDT.get().expect("failed to get IDT").load();
    unsafe {
        let mut pics = PICS.lock();
        pics.initialize();
        // only handle timer interrupts
        pics.write_masks(0b1111_1110, 0b1111_1111);
    }
    interrupts::enable();
    pop_call_stack();
}

extern "x86-interrupt" fn general_handler(stack_frame: InterruptStackFrame, error_code: u64) {
    println!(
        "EXCEPTION: GENERAL_PROTECTION({})\n{:#?}",
        error_code, stack_frame
    );
    panic!();
}

extern "x86-interrupt" fn breakpoint_handler(stack_frame: InterruptStackFrame) {
    println!("EXCEPTION: BREAKPOINT\n{:#?}", stack_frame);
}

extern "x86-interrupt" fn double_handler(stack_frame: InterruptStackFrame, _error_code: u64) -> ! {
    println!("EXCEPTION: DOUBLE FAULT:{:#?}", stack_frame);
    loop {}
}

extern "x86-interrupt" fn timer_handler(_stack_frame: InterruptStackFrame) {
    unsafe {
        PICS.lock()
            .notify_end_of_interrupt(InterruptIndex::Timer as u8)
    }
}

extern "x86-interrupt" fn keyboard_interrupt(_stack_frame: InterruptStackFrame) {
    unsafe {
        PICS.lock()
            .notify_end_of_interrupt(InterruptIndex::Keyboard as u8);
    }
}

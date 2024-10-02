use crate::interrupt::{pop_call_stack, push_call_stack};
use crate::print;
use spin::Once;
use x86_64::instructions::{
    segmentation::{Segment, CS},
    tables::load_tss,
};
use x86_64::structures::{
    gdt::{Descriptor, GlobalDescriptorTable, SegmentSelector},
    tss::TaskStateSegment,
};
use x86_64::VirtAddr;

static TSS: Once<TaskStateSegment> = Once::new();
static GDT: Once<(GlobalDescriptorTable, Selectors)> = Once::new();
pub const DOUBLE_FAULT_1ST_INDEX: u16 = 0;

struct Selectors {
    code_selector: SegmentSelector,
    tss_selector: SegmentSelector,
}

pub fn init_gdt() {
    push_call_stack(0);
    TSS.call_once(|| {
        let mut tss = TaskStateSegment::new();
        const STACK_SIZE: usize = 0x5000;
        static mut STACK: [u8; STACK_SIZE] = [0; STACK_SIZE];

        let stack_start = VirtAddr::from_ptr(unsafe { &raw const STACK });
        let stack_end = stack_start + STACK_SIZE as u64;
        tss.interrupt_stack_table[DOUBLE_FAULT_1ST_INDEX as usize] = stack_end;
        tss
    });

    GDT.call_once(|| {
        let mut gdt = GlobalDescriptorTable::new();
        let code_selector = gdt.append(Descriptor::kernel_code_segment());
        let tss_selector = gdt.append(Descriptor::tss_segment(TSS.get().unwrap()));
        (
            gdt,
            Selectors {
                code_selector,
                tss_selector,
            },
        )
    });

    GDT.get().unwrap().0.load();
    unsafe {
        CS::set_reg(GDT.get().unwrap().1.code_selector);
        load_tss(GDT.get().unwrap().1.tss_selector);
    }
    print!("hi");
    pop_call_stack();
}

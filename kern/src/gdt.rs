use owo_colors::OwoColorize;
use x86_64::{
    instructions::{
        segmentation::{CS, DS, ES, FS, GS, SS},
        tables::load_tss,
    },
    structures::{
        gdt::{Descriptor, GlobalDescriptorTable, SegmentSelector},
        tss::TaskStateSegment,
    },
    VirtAddr,
};

const STACK_SIZE: usize = 1024 * 4;

pub const DOUBLE_FAULT_IST_INDEX: u16 = 0;
pub const PAGE_FAULT_IST_INDEX: u16 = 1;
pub const GENERAL_PROTECTION_FAULT_IST_INDEX: u16 = 2;

macro_rules! tss {
    ($stack:expr) => {{
        let mut tss = TaskStateSegment::new();

        tss.privilege_stack_table[0] = {
            let stack_start = VirtAddr::from_ptr($stack);
            stack_start + STACK_SIZE as u64
        };
        tss.interrupt_stack_table[DOUBLE_FAULT_IST_INDEX as usize] = {
            let stack_start = VirtAddr::from_ptr($stack);
            stack_start + STACK_SIZE as u64
        };
        tss.interrupt_stack_table[PAGE_FAULT_IST_INDEX as usize] = {
            let stack_start = VirtAddr::from_ptr($stack);
            stack_start + STACK_SIZE as u64
        };
        tss.interrupt_stack_table[GENERAL_PROTECTION_FAULT_IST_INDEX as usize] = {
            let stack_start = VirtAddr::from_ptr($stack);
            stack_start + STACK_SIZE as u64
        };

        tss
    }};
}

pub struct Selectors {
    pub kernel_code: SegmentSelector,
    pub kernel_data: SegmentSelector,
    pub user_code: SegmentSelector,
    pub user_data: SegmentSelector,
    pub tss: SegmentSelector,
}

fn build(tss: &'static TaskStateSegment) -> (GlobalDescriptorTable, Selectors) {
    let mut gdt = GlobalDescriptorTable::new();

    let kernel_code = Descriptor::kernel_code_segment();
    let kernel_data = Descriptor::kernel_data_segment();
    let user_code = Descriptor::user_code_segment();
    let user_data = Descriptor::user_data_segment();

    // The order is required.
    let kernel_code_selector = gdt.append(kernel_code);
    let kernel_data_selector = gdt.append(kernel_data);

    let user_data_selector = gdt.append(user_data);
    let user_code_selector = gdt.append(user_code);

    let tss_selector = gdt.append(Descriptor::tss_segment(tss));

    let selectors = Selectors {
        kernel_code: kernel_code_selector,
        kernel_data: kernel_data_selector,
        user_code: user_code_selector,
        user_data: user_data_selector,
        tss: tss_selector,
    };

    (gdt, selectors)
}

fn load(gdt: &'static GlobalDescriptorTable, selectors: &Selectors) {
    gdt.load();

    unsafe {
        use x86_64::instructions::segmentation::Segment;

        CS::set_reg(selectors.kernel_code);
        DS::set_reg(selectors.kernel_data);
        ES::set_reg(selectors.kernel_data);
        FS::set_reg(selectors.kernel_data);
        GS::set_reg(selectors.kernel_data);
        SS::set_reg(selectors.kernel_data);

        load_tss(selectors.tss);
    }
}

lazy_static! {
    pub static ref BP_TSS: TaskStateSegment = tss!({
        static mut STACK: [u8; STACK_SIZE] = [0; STACK_SIZE];
        unsafe { &STACK }
    });
    pub static ref BP_GDT: (GlobalDescriptorTable, Selectors) = build(&BP_TSS);
}

pub fn init() {
    load(&BP_GDT.0, &BP_GDT.1);

    println!("{}: {}", "[TRACE]".yellow(), "GDT initialized");
}

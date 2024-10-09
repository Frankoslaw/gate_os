#![no_std]
#![no_main]
#![allow(internal_features)]
#![feature(abi_x86_interrupt)]
#![feature(alloc_error_handler)]
#![feature(prelude_import)]
#![feature(naked_functions)]
#![feature(const_mut_refs)]
#![feature(never_type)]
#![feature(asm_const)]
#![feature(allocator_api)]
#![feature(ptr_metadata)]
#![feature(slice_ptr_get)]
#![feature(panic_can_unwind)]
#![feature(strict_provenance)]
#![feature(trait_upcasting)]

#[macro_use]
extern crate lazy_static;

#[macro_use]
mod serial;
mod gdt;
// mod memory;
mod allocator;
mod stack_allocator;

mod prelude {
    pub use core::arch::{asm, global_asm};
    pub use core::prelude::v1::*;
    pub use owo_colors::OwoColorize;
}

#[prelude_import]
#[allow(unused_imports)]
use prelude::*;

use raw_cpuid::CpuId;
pub use serial::_print;
use stack_allocator::StackAllocator;
use x86_64::registers::model_specific::Msr;

use limine::request::{
    HhdmRequest, KernelAddressRequest, KernelFileRequest, MemoryMapRequest, RequestsEndMarker,
    RequestsStartMarker, RsdpRequest,
};
use limine::BaseRevision;

#[link_section = ".requests"]
static BASE_REVISION: BaseRevision = BaseRevision::new();

#[link_section = ".requests"]
static MEMMAP: MemoryMapRequest = MemoryMapRequest::new();

#[link_section = ".requests"]
static RSDP: RsdpRequest = RsdpRequest::new();

#[link_section = ".requests"]
static HHDM: HhdmRequest = HhdmRequest::new();

#[link_section = ".requests"]
static KERNEL_FILE: KernelFileRequest = KernelFileRequest::new();

#[link_section = ".requests"]
static KERNEL_ADDRESS: KernelAddressRequest = KernelAddressRequest::new();

#[used]
#[link_section = ".requests_start_marker"]
static _START_MARKER: RequestsStartMarker = RequestsStartMarker::new();
#[used]
#[link_section = ".requests_end_marker"]
static _END_MARKER: RequestsEndMarker = RequestsEndMarker::new();

#[no_mangle]
unsafe extern "C" fn kmain() -> ! {
    assert!(BASE_REVISION.is_supported());

    e9::println!("e9 works :D");

    serial::init();
    gdt::init();

    let memmap = MEMMAP.get_response().unwrap().entries();
    let physical_memory_offset = HHDM.get_response().unwrap().offset();

    // memory::init(physical_memory_offset, memmap);

    // allocator::init();

    println!("{}: {}", "[INFO]".bright_green(), "GateOS (neo-0.1.0)");
    println!();

    println!("DUPA");
    println!("3250 decimal is {:o} octal!", 3250);

    hcf();
}

#[cfg(not(test))]
#[panic_handler]
fn rust_panic(info: &core::panic::PanicInfo) -> ! {
    e9::println!("{}", "[PANIC]:".red());
    e9::dbg!(info);
    println!("{}", "[PANIC]:".red());
    dbg!(info);
    hcf();
}

fn hcf() -> ! {
    loop {
        unsafe {
            #[cfg(target_arch = "x86_64")]
            asm!("hlt");
        }
    }
}

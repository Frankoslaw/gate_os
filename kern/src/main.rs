#![no_std]
#![no_main]

#[macro_use]
mod serial;
mod gdt;

#[macro_use]
extern crate lazy_static;

use limine::memory_map::EntryType;
use owo_colors::OwoColorize;
pub use serial::_print;
use x86_64::registers::model_specific::Msr;

use core::arch::asm;
use core::mem::transmute;

use limine::request::{HhdmRequest, MemoryMapRequest, RequestsEndMarker, RequestsStartMarker};
use limine::BaseRevision;

#[link_section = ".requests"]
static BASE_REVISION: BaseRevision = BaseRevision::new();

#[link_section = ".requests"]
static MEMMAP: MemoryMapRequest = MemoryMapRequest::new();

#[link_section = ".requests"]
static HHDM: HhdmRequest = HhdmRequest::new();

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
    set_pid(0);

    gdt::init();

    let memmap = MEMMAP.get_response().unwrap().entries();
    for entry in memmap {
        let entry_type = unsafe { transmute::<EntryType, u64>(entry.entry_type) };

        println!(
            "{}: Base: {:X}, Type: {:X}, Length: {:X}",
            "[DEBUG]".cyan(),
            entry.base,
            entry_type,
            entry.length
        )
    }

    let physical_memory_offset = HHDM.get_response().unwrap().offset();

    // After init

    println!("{}: {}", "[INFO]".bright_green(), "GateOS (neo-0.1.0)");
    println!();

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

fn set_pid(pid: u64) {
    let mut msr = Msr::new(0xc0000103);
    unsafe {
        msr.write(pid);
    }
}

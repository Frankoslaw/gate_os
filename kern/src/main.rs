#![no_std]
#![no_main]

#[macro_use]
mod serial;

use owo_colors::OwoColorize;
pub use serial::_print;
use x86_64::registers::model_specific::Msr;

use core::arch::asm;

use limine::request::{RequestsEndMarker, RequestsStartMarker};
use limine::BaseRevision;

#[link_section = ".requests"]
static BASE_REVISION: BaseRevision = BaseRevision::new();

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

    println!("{}: {}", "[INFO]".bright_green(), "GateOS (neo-0.1.0)");
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

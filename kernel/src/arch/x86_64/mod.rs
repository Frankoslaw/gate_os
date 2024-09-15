mod framebuffer;
mod gdt;

use core::borrow::BorrowMut;

use limine::request::{FramebufferRequest, HhdmRequest, MemoryMapRequest};
use limine::BaseRevision;
use x86_64::registers::model_specific::Msr;

#[used]
#[link_section = ".requests"]
static BASE_REVISION: BaseRevision = BaseRevision::new();

#[used]
#[link_section = ".requests"]
static FRAMEBUFFER: FramebufferRequest = FramebufferRequest::new();

#[used]
#[link_section = ".requests"]
static HHDM: HhdmRequest = HhdmRequest::new();

#[used]
#[link_section = ".requests"]
static MEMMAP: MemoryMapRequest = MemoryMapRequest::new();

#[no_mangle]
unsafe extern "C" fn kmain() -> ! {
    // TODO: crate::panic::catch_unwind(start);
    start();
}

fn start() -> ! {
    assert!(BASE_REVISION.is_supported());

    init_sse();

    if let Some(framebuffer_response) = FRAMEBUFFER.get_response() {
        if let Some(framebuffer) = framebuffer_response.framebuffers().next() {
            framebuffer::init(&framebuffer);
        }
    }

    set_pid(0);

    gdt::init();

    // let memmap = MEMMAP.get_response().unwrap().borrow_mut();
    // let physical_memory_offset = HHDM.get_response().unwrap().offset();

    crate::main();
}

fn init_sse() {
    // unwinding uses stmxcsr which will UD if this isn't enabled
    unsafe {
        asm!(
            "
            mov rax, cr0
            and ax, 0xFFFB		// clear coprocessor emulation CR0.EM
            or ax, 0x2			  // set coprocessor monitoring  CR0.MP
            mov cr0, rax
            mov rax, cr4
            or ax, 3 << 9		  // set CR4.OSFXSR and CR4.OSXMMEXCPT at the same time
            mov cr4, rax
        ",
            out("rax") _
        );
    }
}

fn set_pid(pid: u64) {
    let mut msr = Msr::new(0xc0000103);
    unsafe {
        msr.write(pid);
    }
}

pub fn get_pid() -> u64 {
    let mut pid;
    unsafe {
        asm!("rdpid {}", out(reg) pid);
    }
    pid
}

pub use framebuffer::_print;

#[inline(always)]
pub fn halt_loop() -> ! {
    loop {
        x86_64::instructions::hlt();
    }
}

#[inline(always)]
pub fn enable_interrupts_and_halt() {
    x86_64::instructions::interrupts::enable_and_hlt();
}

#[allow(unused_imports)]
pub use x86_64::instructions::interrupts::disable as disable_interrupts;
#[allow(unused_imports)]
pub use x86_64::instructions::interrupts::without_interrupts;

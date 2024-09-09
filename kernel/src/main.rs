#![no_std]
#![no_main]

mod debug;
mod framebuffer;
mod logger;

extern crate lazy_static;

use limine::request::FramebufferRequest;
use limine::BaseRevision;

#[used]
#[link_section = ".requests"]
static BASE_REVISION: BaseRevision = BaseRevision::new();

#[used]
#[link_section = ".requests"]
static FRAMEBUFFER_REQUEST: FramebufferRequest = FramebufferRequest::new();

#[no_mangle]
unsafe extern "C" fn kmain() -> ! {
    assert!(BASE_REVISION.is_supported());

    if let Some(framebuffer_response) = FRAMEBUFFER_REQUEST.get_response() {
        let framebuffer = match framebuffer_response.framebuffers().next() {
            Some(i) => i,
            None => halt_loop(),
        };

        framebuffer::init(&framebuffer);
    }

    logger::init();

    println!();
    log::info!("Hello world from Gate OS :D");

    halt_loop();
}

#[cfg(not(test))]
#[panic_handler]
fn rust_panic(_info: &core::panic::PanicInfo) -> ! {
    halt_loop();
}

#[inline(always)]
pub fn halt_loop() -> ! {
    loop {
        x86_64::instructions::hlt();
    }
}

pub use x86_64::instructions::interrupts::disable as disable_interrupts;
pub use x86_64::instructions::interrupts::without_interrupts;

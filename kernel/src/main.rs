#![no_std]
#![no_main]
#![cfg_attr(test, no_main)]
#![feature(custom_test_frameworks)]
#![feature(abi_x86_interrupt)]
#![feature(naked_functions)]
#![feature(alloc_error_handler)]
#![feature(const_mut_refs)]

mod debug;
mod exception_handlers;
mod framebuffer;
mod gdt;
mod idt;
mod logger;

extern crate lazy_static;

use core::mem::transmute;

use limine::memory_map::EntryType;
use limine::request::FramebufferRequest;
use limine::request::KernelAddressRequest;
use limine::request::MemoryMapRequest;
use limine::BaseRevision;

#[used]
#[link_section = ".requests"]
static BASE_REVISION: BaseRevision = BaseRevision::new();

#[used]
#[link_section = ".requests"]
static FRAMEBUFFER_REQUEST: FramebufferRequest = FramebufferRequest::new();

#[used]
#[link_section = ".requests"]
static MEMORY_MAP_REQUEST: MemoryMapRequest = MemoryMapRequest::new();

#[used]
#[link_section = ".requests"]
static KERNEL_ADDRESS_REQUEST: KernelAddressRequest = KernelAddressRequest::new();

#[no_mangle]
unsafe extern "C" fn kmain() -> ! {
    assert!(BASE_REVISION.is_supported());

    if let Some(framebuffer_response) = FRAMEBUFFER_REQUEST.get_response() {
        let framebuffer = match framebuffer_response.framebuffers().next() {
            Some(i) => i,
            None => hlt_loop(),
        };

        framebuffer::init(&framebuffer);
    }

    logger::init();

    gdt::init();
    idt::init();

    disable_interrupts();

    println!();

    if let Some(kernel_adress) = KERNEL_ADDRESS_REQUEST.get_response() {
        log::trace!(
            "Kernel addr - Phys base: {:X}   Virt: {:X}   Rev: {}",
            kernel_adress.physical_base(),
            kernel_adress.virtual_base(),
            kernel_adress.revision()
        );
    }

    println!();

    if let Some(memory_map) = MEMORY_MAP_REQUEST.get_response() {
        let entries_count = memory_map.entries().len();
        log::trace!("Total number of entries: {}", entries_count);

        for (i, entry) in memory_map.entries().iter().enumerate() {
            log::trace!("Processing entry {}", i);

            log::trace!("Entry - Base: {:X}   Len: {:X}", entry.base, entry.length);
        }
    }

    println!();

    println!("Finished");

    hlt_loop();
}

#[cfg(not(test))]
#[panic_handler]
fn rust_panic(_info: &core::panic::PanicInfo) -> ! {
    hlt_loop();
}

#[inline(always)]
pub fn hlt_loop() -> ! {
    loop {
        x86_64::instructions::hlt();
    }
}

pub use x86_64::instructions::interrupts::disable as disable_interrupts;
pub use x86_64::instructions::interrupts::without_interrupts;

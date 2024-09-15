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

mod prelude {
    // pub use alloc::{
    //     borrow::ToOwned,
    //     boxed::Box,
    //     format,
    //     string::{String, ToString},
    //     vec,
    //     vec::Vec,
    // };
    pub use core::arch::{asm, global_asm};
    pub use core::prelude::v1::*;
}

#[prelude_import]
#[allow(unused_imports)]
use prelude::*;

#[macro_use]
mod debug;
mod arch;
mod panic;

pub fn main() -> ! {
    println!("Welcome to Gate OS (neo)");

    arch::halt_loop();
}

// pub fn ap_main(ap_id: u8) -> ! {
//     task::start(ap_id);
//     arch::halt_loop();
// }

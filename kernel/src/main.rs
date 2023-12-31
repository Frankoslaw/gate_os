#![no_std]
#![no_main]

extern crate alloc;

#[allow(unused_imports)]
use core::panic::PanicInfo;

use bootloader_api::config::{BootloaderConfig, Mapping};
use bootloader_api::{entry_point, BootInfo};
use kernel::{print, println, usr, sys};

pub static BOOTLOADER_CONFIG: BootloaderConfig = {
    let mut config = BootloaderConfig::new_default();
    config.mappings.physical_memory = Some(Mapping::Dynamic);
    config
};

entry_point!(kernel_main, config = &BOOTLOADER_CONFIG);

fn kernel_main(boot_info: &'static mut BootInfo) -> ! {
    kernel::init(boot_info);
    print!("\x1b[?25h");


    loop {
        user_boot();
    }
}

fn user_boot() {
    if !sys::fs::is_mounted() {
        println!("MFS is not mounted to '/'");
    }

    usr::shell::main().ok();
}

#[cfg(not(test))]
#[panic_handler]
fn panic(info: &PanicInfo) -> ! {
    log::error!("{}", info);

    loop {}
}

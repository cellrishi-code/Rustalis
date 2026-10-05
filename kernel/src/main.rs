#![no_std]
#![no_main]

use bootloader_api::{entry_point, BootInfo};
use core::panic::PanicInfo;

mod arch;
mod logger;

entry_point!(kernel_main);

fn kernel_main(boot_info: &'static mut BootInfo) -> ! {
    logger::init();
    logger::info("Rustalis kernel starting");
    logger::info("Architecture: x86_64");
    logger::info("Boot information received");
    let _ = boot_info.memory_regions.len();
    logger::info("Memory map is available");
    logger::info("Rustalis booted successfully.");
    loop { core::hint::spin_loop(); }
}

#[panic_handler]
fn panic(info: &PanicInfo) -> ! {
    logger::error("KERNEL PANIC");
    logger::panic_info(info);
    loop { core::hint::spin_loop(); }
}

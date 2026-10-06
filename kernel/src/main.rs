#![feature(abi_x86_interrupt)]

#![no_std]
#![no_main]

use bootloader_api::{entry_point, BootInfo};
use core::panic::PanicInfo;

mod arch;
mod logger;
mod memory;

entry_point!(kernel_main);

fn kernel_main(boot_info: &'static mut BootInfo) -> ! {
    logger::init();
    logger::info("Rustalis kernel starting");
    logger::info("Architecture: x86_64");
    logger::info("Boot information received");
    arch::x86_64::init();
    logger::info("x86_64 architecture layer initialized");
    #[cfg(feature = "exception-self-test")]
    {
        logger::info("Running breakpoint exception self-test");
        arch::x86_64::idt::self_test();
        logger::info("Breakpoint exception self-test passed");
    }
    let summary = memory::summarize(&boot_info.memory_regions);
    logger::info_u64("Memory regions", summary.region_count as u64);
    logger::info_u64("Usable memory bytes", summary.usable_bytes);
    logger::info_u64("Reserved memory bytes", summary.reserved_bytes);
    logger::info("Rustalis booted successfully.");
    loop { core::hint::spin_loop(); }
}

#[panic_handler]
fn panic(info: &PanicInfo) -> ! {
    logger::error("KERNEL PANIC");
    logger::panic_info(info);
    loop { core::hint::spin_loop(); }
}

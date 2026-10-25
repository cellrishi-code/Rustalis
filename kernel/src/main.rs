#![no_std]
#![no_main]

use bootloader_api::{BootInfo, entry_point};
use core::panic::PanicInfo;

mod arch;
mod fs;
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

    let summary = memory::summarize(&boot_info.memory_regions);
    logger::info_u64("Memory regions", summary.region_count as u64);
    logger::info_u64("Usable memory bytes", summary.usable_bytes);
    logger::info_u64("Reserved memory bytes", summary.reserved_bytes);

    // Exercise the allocation-free RAM filesystem before the heap and storage
    // drivers exist. This is volatile memory and is reset on every reboot.
    let mut ramfs = fs::RamFs::<8>::new();
    if ramfs.create("/welcome.txt").is_ok()
        && ramfs.write("/welcome.txt", b"Welcome to Rustalis").is_ok()
        && ramfs.read("/welcome.txt") == Ok(&b"Welcome to Rustalis"[..])
    {
        logger::info("RAM filesystem initialized; /welcome.txt verified");
        logger::info_u64("RAM filesystem files", ramfs.file_count() as u64);
    } else {
        logger::error("RAM filesystem smoke test failed");
    }

    logger::info("Rustalis booted successfully.");
    loop {
        core::hint::spin_loop();
    }
}

#[panic_handler]
fn panic(info: &PanicInfo) -> ! {
    logger::error("KERNEL PANIC");
    logger::panic_info(info);
    loop {
        core::hint::spin_loop();
    }
}

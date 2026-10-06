//! x86_64-specific kernel facilities.

pub mod interrupts;
pub mod idt;

/// Initialize CPU-specific facilities that are safe to establish at boot.
pub fn init() {
    interrupts::init();
}

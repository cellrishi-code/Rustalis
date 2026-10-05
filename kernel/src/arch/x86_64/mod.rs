//! x86_64-specific kernel facilities.

pub mod interrupts;

/// Initialize CPU-specific facilities that are safe to establish at boot.
pub fn init() {
    interrupts::init();
}

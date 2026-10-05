use core::arch::asm;

#[repr(C)]
#[derive(Clone, Copy)]
pub struct InterruptStackFrame {
    pub instruction_pointer: u64,
    pub code_segment: u64,
    pub cpu_flags: u64,
    pub stack_pointer: u64,
    pub stack_segment: u64,
}

/// Architectural interrupt subsystem boundary.
pub fn init() {
    // SAFETY: maskable interrupts stay disabled until an IDT and handlers
    // have been installed. This prevents delivery to an incomplete table.
    unsafe { asm!("cli", options(nomem, nostack, preserves_flags)); }
}

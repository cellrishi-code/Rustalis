//! Interrupt Descriptor Table setup and CPU exception handlers.

use spin::Once;
use x86_64::structures::idt::{InterruptDescriptorTable, InterruptStackFrame};

static IDT: Once<InterruptDescriptorTable> = Once::new();

/// Install the Rustalis exception handlers and load the IDT.
///
/// The IDT is initialized exactly once and stored in a static so its address
/// remains stable for as long as the CPU can dispatch interrupts.
pub fn init() {
    let idt = IDT.call_once(|| {
        let mut idt = InterruptDescriptorTable::new();
        idt.breakpoint.set_handler_fn(breakpoint_handler);
        idt.invalid_opcode.set_handler_fn(invalid_opcode_handler);
        idt
    });

    // SAFETY: IDT lives in a static Once and is therefore never moved or
    // destroyed after initialization. Interrupts remain disabled until the
    // table has been loaded.
    idt.load();
}

/// Handle INT3/#BP without modifying the saved execution state.
extern "x86-interrupt" fn breakpoint_handler(frame: InterruptStackFrame) {
    crate::logger::info("BREAKPOINT exception (#BP)");
    crate::logger::info_u64("Breakpoint RIP", frame.instruction_pointer.as_u64());
    crate::logger::info_u64("Breakpoint CS", frame.code_segment.bits() as u64);
    crate::logger::info_u64("Breakpoint RFLAGS", frame.cpu_flags.bits());
    crate::logger::info_u64("Breakpoint RSP", frame.stack_pointer.as_u64());
    crate::logger::info_u64("Breakpoint SS", frame.stack_segment.bits() as u64);
}

/// Handle an invalid opcode as a controlled kernel failure.
///
/// Returning would retry the faulting instruction because #UD reports the
/// instruction that caused the exception. Entering the kernel panic path
/// avoids silently looping on the same invalid instruction.
extern "x86-interrupt" fn invalid_opcode_handler(frame: InterruptStackFrame) -> ! {
    crate::logger::error("INVALID OPCODE exception (#UD)");
    crate::logger::info_u64("Invalid opcode RIP", frame.instruction_pointer.as_u64());
    crate::logger::info_u64("Invalid opcode CS", frame.code_segment.bits() as u64);
    panic!("invalid opcode exception");
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn idt_registers_exception_handlers() {
        let mut idt = InterruptDescriptorTable::new();
        idt.breakpoint.set_handler_fn(breakpoint_handler);
        idt.invalid_opcode.set_handler_fn(invalid_opcode_handler);

        assert_ne!(idt.breakpoint.handler_addr().as_u64(), 0);
        assert_ne!(idt.invalid_opcode.handler_addr().as_u64(), 0);
    }
}

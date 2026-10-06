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
        idt.divide_error.set_handler_fn(divide_error_handler);
        idt.breakpoint.set_handler_fn(breakpoint_handler);
        idt.invalid_opcode.set_handler_fn(invalid_opcode_handler);
        idt
    });

    // SAFETY: IDT lives in a static Once and is therefore never moved or
    // destroyed after initialization. Interrupts remain disabled until the
    // table has been loaded.
    idt.load();
}

/// Trigger INT3 for the optional boot-time exception self-test.
#[cfg(feature = "exception-self-test")]
pub fn self_test() {
    x86_64::instructions::interrupts::int3();
}

/// Trigger a divide-error exception for the optional integration self-test.
///
/// This deliberately executes an integer divide by zero. It must only be
/// called after the IDT has been loaded and only from the self-test path.
#[cfg(feature = "divide-error-self-test")]
pub fn divide_error_self_test() {
    // SAFETY: This inline assembly intentionally triggers #DE to verify the
    // installed handler. The feature is opt-in and the handler terminates
    // through the kernel panic path instead of returning to this instruction.
    unsafe {
        core::arch::asm!(
            "div rcx",
            inlateout("rax") 1u64 => _,
            inlateout("rdx") 0u64 => _,
            in("rcx") 0u64,
            options(nostack),
        );
    }
}

/// Handle integer divide errors as a controlled kernel failure.
extern "x86-interrupt" fn divide_error_handler(frame: InterruptStackFrame) -> ! {
    crate::logger::error("DIVIDE ERROR exception (#DE)");
    crate::logger::info_u64("Divide error RIP", frame.instruction_pointer.as_u64());
    crate::logger::info_u64("Divide error CS", frame.code_segment.bits() as u64);
    panic!("divide error exception");
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
        idt.divide_error.set_handler_fn(divide_error_handler);
        idt.breakpoint.set_handler_fn(breakpoint_handler);
        idt.invalid_opcode.set_handler_fn(invalid_opcode_handler);

        assert_ne!(idt.divide_error.handler_addr().as_u64(), 0);
        assert_ne!(idt.breakpoint.handler_addr().as_u64(), 0);
        assert_ne!(idt.invalid_opcode.handler_addr().as_u64(), 0);
    }
}

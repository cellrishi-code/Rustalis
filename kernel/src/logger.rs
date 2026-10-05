use core::fmt::Write;
use spin::Mutex;
use uart_16550::SerialPort;

static SERIAL1: Mutex<Option<SerialPort>> = Mutex::new(None);

pub fn init() {
    // SAFETY: 0x3F8 is the conventional COM1 I/O base used by the x86 PC
    // platform. The serial device is initialized once during single-core boot,
    // before concurrent kernel subsystems are started.
    let mut serial = unsafe { SerialPort::new(0x3F8) };
    serial.init();
    *SERIAL1.lock() = Some(serial);
}

fn write_line(level: &str, message: &str) {
    if let Some(serial) = SERIAL1.lock().as_mut() {
        let _ = writeln!(serial, "[{level}] {message}");
    }
}

pub fn info(message: &str) {
    write_line("INFO", message);
}

pub fn error(message: &str) {
    write_line("ERROR", message);
}

pub fn panic_info(info: &core::panic::PanicInfo<'_>) {
    if let Some(serial) = SERIAL1.lock().as_mut() {
        let _ = writeln!(serial, "{info}");
    }
}

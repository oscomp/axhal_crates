//! Uart 16550 serial port.

use axhal_plat::console::ConsoleIf;
use kspin::SpinNoIrq;
use lazyinit::LazyInit;
use uart_16550::SerialPort;

static COM1: LazyInit<SpinNoIrq<SerialPort>> = LazyInit::new();

/// Writes a byte to the console.
pub fn putchar(c: u8) {
    COM1.lock().send(c)
}

/// Reads a byte from the console, or returns [`None`] if no input is available.
pub fn getchar() -> Option<u8> {
    COM1.lock().try_receive().ok()
}

/// Initialize COM1 serial port
///
/// When we invoke this function, the irq is not enabled yet.
/// So we must finish the initialization of COM1 without locking the `SpinNoIrq`.
pub fn init() {
    let mut com1 = unsafe { SerialPort::new(0x3f8) };
    com1.init();
    COM1.init_once(SpinNoIrq::new(com1));
}

struct ConsoleIfImpl;

#[impl_plat_interface]
impl ConsoleIf for ConsoleIfImpl {
    /// Writes given bytes to the console.
    fn write_bytes(bytes: &[u8]) {
        for c in bytes {
            putchar(*c);
        }
    }

    /// Reads bytes from the console into the given mutable slice.
    ///
    /// Returns the number of bytes read.
    fn read_bytes(bytes: &mut [u8]) -> usize {
        let mut read_len = 0;
        while read_len < bytes.len() {
            if let Some(c) = getchar() {
                bytes[read_len] = c;
            } else {
                break;
            }
            read_len += 1;
        }
        read_len
    }
}

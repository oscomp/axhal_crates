//! PL011 UART.

use arm_pl011::Pl011Uart;
use kspin::SpinNoIrq;
use lazyinit::LazyInit;
use memory_addr::VirtAddr;

static UART: LazyInit<SpinNoIrq<Pl011Uart>> = LazyInit::new();

/// Writes a byte to the console.
pub fn putchar(c: u8) {
    let mut uart = UART.lock();
    match c {
        b'\n' => {
            uart.putchar(b'\r');
            uart.putchar(b'\n');
        }
        c => uart.putchar(c),
    }
}

/// Reads a byte from the console, or returns [`None`] if no input is available.
fn getchar() -> Option<u8> {
    UART.lock().getchar()
}

/// Write a slice of bytes to the console.
pub fn write_bytes(bytes: &[u8]) {
    for c in bytes {
        putchar(*c);
    }
}

/// Reads bytes from the console into the given mutable slice.
/// Returns the number of bytes read.
pub fn read_bytes(bytes: &mut [u8]) -> usize {
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

/// Early stage initialization of the PL011 UART driver.
pub fn init_early(uart_base: VirtAddr) {
    UART.init_once(SpinNoIrq::new(Pl011Uart::new(uart_base.as_mut_ptr())));
    UART.lock().init();
}

/// UART IRQ Handler
pub fn irq_handler() {
    let is_receive_interrupt = UART.lock().is_receive_interrupt();
    UART.lock().ack_interrupts();
    if is_receive_interrupt {
        while let Some(c) = getchar() {
            putchar(c);
        }
    }
}

/// Default implementation of [`axhal_plat::console::ConsoleIf`] using the
/// PL011 UART.
#[macro_export]
macro_rules! console_if_impl {
    ($name:ident) => {
        struct $name;

        #[axhal_plat::impl_plat_interface]
        impl axhal_plat::console::ConsoleIf for $name {
            /// Writes given bytes to the console.
            fn write_bytes(bytes: &[u8]) {
                $crate::pl011::write_bytes(bytes);
            }

            /// Reads bytes from the console into the given mutable slice.
            ///
            /// Returns the number of bytes read.
            fn read_bytes(bytes: &mut [u8]) -> usize {
                $crate::pl011::read_bytes(bytes)
            }
        }
    };
}

use crate::mem::phys_to_virt;
use kspin::SpinNoIrq;
use lazyinit::LazyInit;
use memory_addr::PhysAddr;
use ns16550a::Uart;

const UART_BASE: PhysAddr = pa!(crate::config::devices::UART_PADDR);

static UART: LazyInit<SpinNoIrq<Uart>> = LazyInit::new();

/// Early stage initialization for ns16550a
pub(super) fn init_early() {
    let vaddr = phys_to_virt(UART_BASE);
    UART.init_once(SpinNoIrq::new(Uart::new(vaddr.as_usize())));
}

use axhal_plat::console::ConsoleIf;

struct ConsoleIfImpl;

#[impl_plat_interface]
impl ConsoleIf for ConsoleIfImpl {
    /// Writes bytes to the console from input u8 slice.
    fn write_bytes(bytes: &[u8]) {
        for &c in bytes {
            let uart = UART.lock();
            match c {
                b'\n' => {
                    let _ = uart.put(b'\r');
                    let _ = uart.put(b'\n');
                }
                c => {
                    let _ = uart.put(c);
                }
            }
        }
    }

    /// Reads bytes from the console into the given mutable slice.
    /// Returns the number of bytes read.
    fn read_bytes(bytes: &mut [u8]) -> usize {
        for (i, byte) in bytes.iter_mut().enumerate() {
            match UART.lock().get() {
                Some(c) => *byte = c,
                None => return i,
            }
        }
        bytes.len()
    }
}

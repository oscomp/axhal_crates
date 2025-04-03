//! Power management.

use axhal_plat::power::PowerIf;
use x86_64::instructions::port::PortWriteOnly;

struct PowerImpl;

#[impl_plat_interface]
impl PowerIf for PowerImpl {
    /// Bootstraps the given CPU core with the given initial stack (in physical
    /// address).
    ///
    /// Where `cpu_id` is the logical CPU ID (0, 1, ..., N-1, N is the number of
    /// CPU cores on the platform).
    fn cpu_boot(_cpu_id: usize, _stack_top_paddr: usize) {
        #[cfg(feature = "smp")]
        crate::mp::start_secondary_cpu(_cpu_id, pa!(_stack_top_paddr));
    }

    /// Shutdown the whole system (in QEMU).
    ///
    /// See <https://wiki.osdev.org/Shutdown> for more information.
    fn system_off() -> ! {
        info!("Shutting down...");

        // For real hardware platforms, using port `0x604` to shutdown does not
        // work. Therefore we use port `0x64` to reboot the system instead.
        if cfg!(feature = "reboot-on-system-off") {
            axhal_plat::console_println!("System will reboot, press any key to continue ...");
            while super::console::getchar().is_none() {}
            axhal_plat::console_println!("Rebooting ...");
            unsafe { PortWriteOnly::new(0x64).write(0xfeu8) };
        } else {
            unsafe { PortWriteOnly::new(0x604).write(0x2000u16) };
        }

        axhal_cpu::halt();
        warn!("It should shutdown!");
        loop {
            axhal_cpu::halt();
        }
    }
}

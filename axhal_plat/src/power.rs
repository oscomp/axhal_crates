//! Power management.

/// Power management interface.
#[def_plat_interface]
pub trait PowerIf {
    /// Bootstraps the given CPU with the given initial stack (in physical address).
    ///
    /// Where `cpu_id` is the logical CPU ID (0, 1, ..., N-1, N is the number of
    /// CPU cores on the platform).
    fn cpu_boot(cpu_id: usize, stack_top_paddr: usize);

    /// Shutdown the whole system, including all CPUs.
    fn system_off() -> !;
}

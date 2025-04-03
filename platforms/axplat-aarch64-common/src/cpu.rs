//! ARM CPU initialization

/// Initializes the CPU for the primary core.
///
/// This function is called by the bootloader to disable low address access and initialize
/// `percpu` for the primary core.
pub fn init_cpu_primary(cpu_id: usize) {
    unsafe { axhal_cpu::write_page_table_root0(0.into()) }; // disable low address access
    percpu::init();
    percpu::init_percpu_reg(cpu_id);
}

/// Initializes the CPU for the secondary core.
///
/// This function is called by the bootloader to disable low address access and initialize
/// `percpu` for the secondary core.
#[cfg(feature = "smp")]
pub fn init_cpu_secondary(cpu_id: usize) {
    unsafe { axhal_cpu::write_page_table_root0(0.into()) }; // disable low address access
    percpu::init_percpu_reg(cpu_id);
}

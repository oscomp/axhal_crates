pub fn init_cpu_primary(cpu_id: usize) {
    unsafe { axhal_cpu::write_page_table_root0(0.into()) }; // disable low address access
    percpu::init();
    percpu::init_percpu_reg(cpu_id);
}

#[cfg(feature = "smp")]
pub fn init_cpu_secondary(cpu_id: usize) {
    unsafe { axhal_cpu::write_page_table_root0(0.into()) }; // disable low address access
    percpu::init_percpu_reg(cpu_id);
}

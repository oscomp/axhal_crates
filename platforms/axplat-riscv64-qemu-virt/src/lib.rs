#![no_std]
#![feature(naked_functions)]

#[macro_use]
extern crate log;
#[macro_use]
extern crate axhal_plat;
#[macro_use]
extern crate memory_addr;

mod boot;
mod console;
mod init;
#[cfg(feature = "irq")]
mod irq;
mod mem;
mod power;
mod time;

mod config {
    axconfig_gen_macros::include_configs!("axconfig.toml");
}

unsafe extern "C" {
    fn trap_vector_base();
}

unsafe extern "C" fn rust_entry(cpu_id: usize, dtb: usize) {
    unsafe { axhal_plat::mem::clear_bss() };
    init_cpu_primary(cpu_id);
    self::time::init_early();
    axhal_plat::call_main(cpu_id, dtb);
}

#[cfg(feature = "smp")]
unsafe extern "C" fn rust_entry_secondary(cpu_id: usize) {
    init_cpu_secondary(cpu_id);
    axhal_plat::call_secondary_main(cpu_id);
}

fn init_cpu_primary(cpu_id: usize) {
    axhal_cpu::set_trap_vector_base(trap_vector_base as usize);
    percpu::init();
    percpu::init_percpu_reg(cpu_id);
}

#[cfg(feature = "smp")]
fn init_cpu_secondary(cpu_id: usize) {
    axhal_cpu::set_trap_vector_base(trap_vector_base as usize);
    percpu::init_percpu_reg(cpu_id);
}

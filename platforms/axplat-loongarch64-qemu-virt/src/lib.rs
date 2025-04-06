#![no_std]
#![feature(naked_functions)]

#[macro_use]
extern crate log;
#[macro_use]
extern crate axhal_plat;
#[macro_use]
extern crate memory_addr;

mod config {
    axconfig_gen_macros::include_configs!("axconfig.toml");
}

mod boot;
mod console;
mod init;
// #[cfg(feature = "irq")]
mod irq;
mod mem;
#[cfg(feature = "smp")]
mod mp;
mod power;
mod time;

use axhal_cpu::set_exception_entry_base;

fn init_cpu_primary(cpu_id: usize) {
    percpu::init();
    percpu::init_percpu_reg(cpu_id);
    #[cfg(feature = "fp_simd")]
    loongArch64::register::euen::set_fpe(true);

    unsafe extern "C" {
        fn exception_entry_base();
    }
    set_exception_entry_base(exception_entry_base as usize);
}

#[cfg(feature = "smp")]
fn init_cpu_secondary(cpu_id: usize) {
    percpu::init_percpu_reg(cpu_id);
    #[cfg(feature = "fp_simd")]
    loongArch64::register::euen::set_fpe(true);

    unsafe extern "C" {
        fn exception_entry_base();
    }
    set_exception_entry_base(exception_entry_base as usize);
}

/// Rust temporary entry point
///
/// This function will be called after assembly boot stage.
unsafe extern "C" fn rust_entry(cpu_id: usize) {
    unsafe { axhal_plat::mem::clear_bss() };
    crate::console::init_early();
    init_cpu_primary(cpu_id);
    crate::time::init_primary();
    axhal_plat::call_main(cpu_id, 0);
}

#[cfg(feature = "smp")]
/// The entry point for the second core.
pub(crate) extern "C" fn rust_entry_secondary(cpu_id: usize) {
    init_cpu_secondary(cpu_id);
    axhal_plat::call_secondary_main(cpu_id);
}

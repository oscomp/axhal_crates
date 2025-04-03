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
mod dw_apb_uart;
mod init;
mod mem;
mod misc;
#[cfg(feature = "smp")]
mod mp;
mod power;

unsafe extern "C" {
    fn exception_vector_base();
}
use crate::config::plat::PSCI_METHOD;

axplat_aarch64_common::time_if_impl!(TimeIfImpl);

#[cfg(feature = "irq")]
axplat_aarch64_common::irq_if_impl!(IrqIfImpl);

pub(crate) unsafe extern "C" fn rust_entry(cpu_id: usize, dtb: usize) {
    unsafe { axhal_plat::mem::clear_bss() };
    axhal_cpu::set_exception_vector_base(exception_vector_base as usize);
    axplat_aarch64_common::cpu::init_cpu_primary(cpu_id);
    axplat_aarch64_common::psci::init(PSCI_METHOD);
    dw_apb_uart::init_early();
    axplat_aarch64_common::generic_timer::init_early();
    axhal_plat::call_main(cpu_id, dtb);
}

#[cfg(feature = "smp")]
pub(crate) unsafe extern "C" fn rust_entry_secondary(cpu_id: usize) {
    axhal_cpu::set_exception_vector_base(exception_vector_base as usize);
    axplat_aarch64_common::cpu::init_cpu_secondary(cpu_id);
    axhal_plat::call_secondary_main(cpu_id);
}

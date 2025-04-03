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
mod init;
mod mem;
mod power;

use mem::phys_to_virt;

#[allow(unused_imports)]
use self::config::devices::UART_PADDR;
use self::config::plat::PSCI_METHOD;

unsafe extern "C" {
    fn exception_vector_base();
}

axplat_aarch64_common::console_if_impl!(ConsoleIfImpl);
axplat_aarch64_common::time_if_impl!(TimeIfImpl);

#[cfg(feature = "irq")]
axplat_aarch64_common::irq_if_impl!(IrqIfImpl);

pub(crate) unsafe extern "C" fn rust_entry(cpu_id: usize, dtb: usize) {
    unsafe { axhal_plat::mem::clear_bss() };
    axhal_cpu::set_exception_vector_base(exception_vector_base as usize);
    axplat_aarch64_common::cpu::init_cpu_primary(cpu_id);
    axplat_aarch64_common::psci::init(PSCI_METHOD);
    axplat_aarch64_common::pl011::init_early(phys_to_virt(pa!(UART_PADDR)));
    axplat_aarch64_common::generic_timer::init_early();
    let cpu_id = cpu_hard_id_to_logic_id(cpu_id);
    axhal_plat::call_main(cpu_id, dtb);
}

#[cfg(feature = "smp")]
pub(crate) unsafe extern "C" fn rust_entry_secondary(cpu_id: usize) {
    axhal_cpu::set_exception_vector_base(exception_vector_base as usize);
    axplat_aarch64_common::cpu::init_cpu_secondary(cpu_id);
    let cpu_id = cpu_hard_id_to_logic_id(cpu_id);
    axhal_plat::call_secondary_main(cpu_id);
}

fn cpu_hard_id_to_logic_id(hard_id: usize) -> usize {
    crate::config::devices::CPU_ID_LIST
        .iter()
        .position(|&x| x == hard_id)
        .unwrap()
}

//! Description tables (per-CPU GDT, per-CPU ISS, IDT)

use axhal_cpu::{init_gdt, init_idt};

fn init_percpu() {
    percpu::init_percpu_reg(super::current_cpu_id());
    init_idt();
    init_gdt();
}

/// Initializes IDT, GDT on the primary CPU.
pub fn init_primary() {
    percpu::init();
    init_percpu();
    axhal_plat::console_println!("\nFinish initialize IDT & GDT.");
}

/// Initializes IDT, GDT on secondary CPUs.
#[cfg(feature = "smp")]
pub fn init_secondary() {
    init_percpu();
}

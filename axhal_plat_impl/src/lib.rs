//! Top-level feature selection for [axhal_crates].
//!
//! # Platform Features
//!
//! - x86-pc
//! - riscv64-qemu-virt
//! - aarch64-bsta1000b
//! - aarch64-phytium-pi
//! - aarch64-qemu-virt
//! - aarch64-raspi
//!
//! [ArceOS]: https://github.com/arceos-org/arceos
//! [axhal_crates]: https://github.com/arceos-org/axhal_crates

#![no_std]

#[cfg(target_arch = "aarch64")]
extern crate axplat_aarch64_common;

cfg_if::cfg_if! {
    if #[cfg(all(target_arch = "x86_64", feature = "x86-pc"))] {
        extern crate axplat_x86_pc;
    } else if #[cfg(all(target_arch = "riscv64", feature = "riscv64-qemu-virt"))] {
        extern crate axplat_riscv64_qemu_virt;
    } else if #[cfg(all(target_arch = "aarch64", feature = "aarch64-qemu-virt"))] {
        extern crate axplat_aarch64_qemu_virt;
    } else if #[cfg(all(target_arch = "aarch64", feature = "aarch64-raspi"))] {
        extern crate axplat_aarch64_raspi;
    } else if #[cfg(all(target_arch = "aarch64", feature = "aarch64-bsta1000b"))] {
        extern crate axplat_aarch64_bsta1000b;
    } else if #[cfg(all(target_arch = "aarch64", feature = "aarch64-phytium-pi"))] {
        extern crate axplat_aarch64_phytium_pi;
    } else {
        extern crate axplat_dummy;
    }
}

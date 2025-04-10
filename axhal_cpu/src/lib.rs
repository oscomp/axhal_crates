//! Architecture-specific types and operations.

#![no_std]
#![feature(naked_functions)]

#[macro_use]
extern crate log;

#[macro_use]
extern crate memory_addr;

pub mod cpu;

#[macro_use]
pub mod trap;

cfg_if::cfg_if! {
    if #[cfg(target_arch = "x86_64")] {
        mod x86_64;
        pub use self::x86_64::*;
    } else if #[cfg(any(target_arch = "riscv32", target_arch = "riscv64"))] {
        mod riscv;
        pub use self::riscv::*;
    } else if #[cfg(target_arch = "aarch64")]{
        mod aarch64;
        pub use self::aarch64::*;
    } else if #[cfg(target_arch = "loongarch64")] {
        mod loongarch64;
        pub use self::loongarch64::*;
    }
}

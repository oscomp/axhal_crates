#![no_std]

#[macro_use]
extern crate axhal_plat;

mod console;
mod init;
mod irq;
mod mem;
mod power;
mod time;

mod config {
    axconfig_gen_macros::include_configs!("axconfig.toml");
}

#[cfg(not(test))]
unsafe extern "C" fn _start() -> ! {
    // TODO: Implement actual bootstrap logic
    axhal_plat::call_main(0, 0);
}

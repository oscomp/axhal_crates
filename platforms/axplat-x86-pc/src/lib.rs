#![no_std]
#![feature(sync_unsafe_cell)]

#[macro_use]
extern crate log;
#[macro_use]
extern crate memory_addr;
#[macro_use]
extern crate axhal_plat;

mod apic;
mod boot;
mod console;
mod dtables;
mod init;
mod mem;
mod power;
mod time;

#[cfg(feature = "smp")]
mod mp;

mod config {
    axconfig_gen_macros::include_configs!("axconfig.toml");
}

fn current_cpu_id() -> usize {
    match raw_cpuid::CpuId::new().get_feature_info() {
        Some(finfo) => finfo.initial_local_apic_id() as usize,
        None => 0,
    }
}

unsafe extern "C" fn rust_entry(magic: usize, mbi: usize) {
    // TODO: handle multiboot info
    if magic == self::boot::MULTIBOOT_BOOTLOADER_MAGIC {
        unsafe { axhal_plat::mem::clear_bss() };
        self::console::init();
        self::dtables::init_primary();
        self::time::init_early();
        self::mem::init(mbi);
        axhal_plat::call_main(current_cpu_id(), 0);
    }
}

unsafe extern "C" fn rust_entry_secondary(_magic: usize) {
    #[cfg(feature = "smp")]
    if _magic == self::boot::MULTIBOOT_BOOTLOADER_MAGIC {
        self::dtables::init_secondary();
        axhal_plat::call_secondary_main(current_cpu_id());
    }
}

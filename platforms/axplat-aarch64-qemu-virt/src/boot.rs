use page_table_entry::{aarch64::A64PTE, GenericPTE, MappingFlags};

use crate::config::plat::{BOOT_STACK_SIZE, PHYS_VIRT_OFFSET};

#[unsafe(link_section = ".bss.stack")]
static mut BOOT_STACK: [u8; BOOT_STACK_SIZE] = [0; BOOT_STACK_SIZE];

#[unsafe(link_section = ".data.boot_page_table")]
static mut BOOT_PT_L0: [A64PTE; 512] = [A64PTE::empty(); 512];

#[unsafe(link_section = ".data.boot_page_table")]
static mut BOOT_PT_L1: [A64PTE; 512] = [A64PTE::empty(); 512];

unsafe fn init_boot_page_table() {
    unsafe {
        // 0x0000_0000_0000 ~ 0x0080_0000_0000, table
        BOOT_PT_L0[0] = A64PTE::new_table(pa!(&raw mut BOOT_PT_L1 as usize));
        // 0x0000_0000_0000..0x0000_4000_0000, 1G block, device memory
        BOOT_PT_L1[0] = A64PTE::new_page(
            pa!(0),
            MappingFlags::READ | MappingFlags::WRITE | MappingFlags::DEVICE,
            true,
        );
        // 0x0000_4000_0000..0x0000_8000_0000, 1G block, normal memory
        BOOT_PT_L1[1] = A64PTE::new_page(
            pa!(0x4000_0000),
            MappingFlags::READ | MappingFlags::WRITE | MappingFlags::EXECUTE,
            true,
        );
    }
}

/// Kernel entry point with Linux image header.
///
/// Some bootloaders require this header to be present at the beginning of the
/// kernel image.
///
/// Documentation: <https://docs.kernel.org/arch/arm64/booting.html>
#[naked]
#[unsafe(no_mangle)]
#[unsafe(link_section = ".text.boot")]
unsafe extern "C" fn _start() -> ! {
    const FLAG_LE: usize = 0b0;
    const FLAG_PAGE_SIZE_4K: usize = 0b10;
    const FLAG_ANY_MEM: usize = 0b1000;
    unsafe {
        // PC = bootloader load address
        // X0 = dtb
        core::arch::naked_asm!("
            add     x13, x18, #0x16     // 'MZ' magic
            b       {entry}             // Branch to kernel start, magic

            .quad   0                   // Image load offset from start of RAM, little-endian
            .quad   _ekernel - _start   // Effective size of kernel image, little-endian
            .quad   {flags}             // Kernel flags, little-endian
            .quad   0                   // reserved
            .quad   0                   // reserved
            .quad   0                   // reserved
            .ascii  \"ARM\\x64\"        // Magic number
            .long   0                   // reserved (used for PE COFF offset)",
            flags = const FLAG_LE | FLAG_PAGE_SIZE_4K | FLAG_ANY_MEM,
            entry = sym _start_primary,
        )
    }
}

/// The earliest entry point for the primary CPU.
#[naked]
#[unsafe(link_section = ".text.boot")]
unsafe extern "C" fn _start_primary() -> ! {
    unsafe {
        // X0 = dtb
        core::arch::naked_asm!("
            mrs     x19, mpidr_el1
            and     x19, x19, #0xffffff     // get current CPU id
            mov     x20, x0                 // save DTB pointer

            adrp    x8, {boot_stack}        // setup boot stack
            add     x8, x8, {boot_stack_size}
            mov     sp, x8

            bl      {switch_to_el1}         // switch to EL1
            bl      {enable_fp}             // enable fp/neon
            bl      {init_boot_page_table}
            adrp    x0, {boot_pt}           // load boot page table
            bl      {enable_mmu}            // setup MMU

            mov     x8, {phys_virt_offset}  // set SP to the high address
            add     sp, sp, x8

            mov     x0, x19                 // call rust_entry(cpu_id, dtb)
            mov     x1, x20
            ldr     x8, ={entry}
            blr     x8
            b      .",
            switch_to_el1 = sym axhal_cpu::switch_to_el1,
            enable_mmu = sym axhal_cpu::enable_mmu,
            enable_fp = sym axhal_cpu::enable_fp,
            init_boot_page_table = sym init_boot_page_table,
            boot_stack = sym BOOT_STACK,
            boot_stack_size = const BOOT_STACK_SIZE,
            boot_pt = sym BOOT_PT_L0,
            phys_virt_offset = const PHYS_VIRT_OFFSET,
            entry = sym crate::rust_entry,
        )
    }
}

/// The earliest entry point for the secondary CPUs.
#[cfg(feature = "smp")]
#[naked]
#[unsafe(link_section = ".text.boot")]
pub(crate) unsafe extern "C" fn _start_secondary() -> ! {
    unsafe {
        // X0 = stack pointer
        core::arch::naked_asm!("
            mrs     x19, mpidr_el1
            and     x19, x19, #0xffffff     // get current CPU id

            mov     sp, x0
            bl      {switch_to_el1}
            adrp    x0, {boot_pt}
            bl      {enable_mmu}
            bl      {enable_fp}

            mov     x8, {phys_virt_offset}  // set SP to the high address
            add     sp, sp, x8

            mov     x0, x19                 // call rust_entry_secondary(cpu_id)
            ldr     x8, ={entry}
            blr     x8
            b      .",
            switch_to_el1 = sym axhal_cpu::switch_to_el1,
            enable_mmu = sym axhal_cpu::enable_mmu,
            enable_fp = sym axhal_cpu::enable_fp,
            boot_pt = sym BOOT_PT_L0,
            phys_virt_offset = const PHYS_VIRT_OFFSET,
            entry = sym crate::rust_entry_secondary,
        )
    }
}

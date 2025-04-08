mod context;

#[cfg(target_os = "none")]
mod trap;

use crate::cpu::CURRENT_TASK_PTR;
use aarch64_cpu::{asm::barrier, registers::*};
use core::arch::asm;
use memory_addr::{PhysAddr, VirtAddr};
use tock_registers::interfaces::{Readable, Writeable};

pub use self::context::{FpState, TaskContext, TrapFrame};

/// Stores the pointer to the current task in the SP_EL0 register.
///
/// In aarch64 architecture, we use `SP_EL0` as the read cache for
/// the current task pointer. And this function will update this cache.
pub(crate) unsafe fn cache_current_task_ptr() {
    unsafe { aarch64_cpu::registers::SP_EL0.set(CURRENT_TASK_PTR.read_current_raw() as u64) };
}

/// Allows the current CPU to respond to interrupts.
#[inline]
pub fn enable_irqs() {
    unsafe { asm!("msr daifclr, #2") };
}

/// Makes the current CPU to ignore interrupts.
#[inline]
pub fn disable_irqs() {
    unsafe { asm!("msr daifset, #2") };
}

/// Returns whether the current CPU is allowed to respond to interrupts.
#[inline]
pub fn irqs_enabled() -> bool {
    !DAIF.matches_all(DAIF::I::Masked)
}

/// Relaxes the current CPU and waits for interrupts.
///
/// It must be called with interrupts enabled, otherwise it will never return.
#[inline]
pub fn wait_for_irqs() {
    aarch64_cpu::asm::wfi();
}

/// Halt the current CPU.
#[inline]
pub fn halt() {
    disable_irqs();
    aarch64_cpu::asm::wfi(); // should never return
}

/// Reads the register that stores the current page table root.
///
/// Returns the physical address of the page table root.
#[inline]
pub fn read_page_table_root() -> PhysAddr {
    let root = TTBR1_EL1.get();
    pa!(root as usize)
}

/// Reads the `TTBR0_EL1` register.
pub fn read_page_table_root0() -> PhysAddr {
    let root = TTBR0_EL1.get();
    pa!(root as usize)
}

/// Writes the register to update the current page table root.
///
/// # Safety
///
/// This function is unsafe as it changes the virtual memory address space.
pub unsafe fn write_page_table_root(root_paddr: PhysAddr) {
    let old_root = read_page_table_root();
    trace!("set page table root: {:#x} => {:#x}", old_root, root_paddr);
    if old_root != root_paddr {
        // kernel space page table use TTBR1 (0xffff_0000_0000_0000..0xffff_ffff_ffff_ffff)
        TTBR1_EL1.set(root_paddr.as_usize() as _);
        flush_tlb(None);
    }
}

/// Writes the `TTBR0_EL1` register.
///
/// # Safety
///
/// This function is unsafe as it changes the virtual memory address space.
pub unsafe fn write_page_table_root0(root_paddr: PhysAddr) {
    TTBR0_EL1.set(root_paddr.as_usize() as _);
    flush_tlb(None);
}

/// Flushes the TLB.
///
/// If `vaddr` is [`None`], flushes the entire TLB. Otherwise, flushes the TLB
/// entry that maps the given virtual address.
#[inline]
pub fn flush_tlb(vaddr: Option<VirtAddr>) {
    unsafe {
        if let Some(vaddr) = vaddr {
            asm!("tlbi vaae1is, {}; dsb sy; isb", in(reg) vaddr.as_usize())
        } else {
            // flush the entire TLB
            asm!("tlbi vmalle1; dsb sy; isb")
        }
    }
}

/// Flushes the entire instruction cache.
#[inline]
pub fn flush_icache_all() {
    unsafe { asm!("ic iallu; dsb sy; isb") };
}

/// Sets the base address of the exception vector (writes `VBAR_EL1`).
#[inline]
pub fn set_exception_vector_base(vbar_el1: usize) {
    VBAR_EL1.set(vbar_el1 as _);
}

/// Flushes the data cache line (64 bytes) at the given virtual address
#[inline]
pub fn flush_dcache_line(vaddr: VirtAddr) {
    unsafe { asm!("dc ivac, {0:x}; dsb sy; isb", in(reg) vaddr.as_usize()) };
}

/// Reads the thread pointer of the current CPU.
///
/// It is used to implement TLS (Thread Local Storage).
#[inline]
pub fn read_thread_pointer() -> usize {
    TPIDR_EL0.get() as usize
}

/// Writes the thread pointer of the current CPU.
///
/// It is used to implement TLS (Thread Local Storage).
///
/// # Safety
///
/// This function is unsafe as it changes the CPU states.
#[inline]
pub unsafe fn write_thread_pointer(tpidr_el0: usize) {
    TPIDR_EL0.set(tpidr_el0 as _)
}

/// Swtich current exception level to EL1.
///
/// It usually used in the kernel booting process, where the kernel starts at
/// EL2 or EL3. At that time, the MMU is not enable and the kernel is using
/// physical address.
///
/// # Safety
///
/// This function is unsafe as it changes the CPU mode.
pub unsafe fn switch_to_el1() {
    SPSel.write(SPSel::SP::ELx);
    SP_EL0.set(0);
    let current_el = CurrentEL.read(CurrentEL::EL);
    if current_el >= 2 {
        if current_el == 3 {
            // Set EL2 to 64bit and enable the HVC instruction.
            SCR_EL3.write(
                SCR_EL3::NS::NonSecure + SCR_EL3::HCE::HvcEnabled + SCR_EL3::RW::NextELIsAarch64,
            );
            // Set the return address and exception level.
            SPSR_EL3.write(
                SPSR_EL3::M::EL1h
                    + SPSR_EL3::D::Masked
                    + SPSR_EL3::A::Masked
                    + SPSR_EL3::I::Masked
                    + SPSR_EL3::F::Masked,
            );
            ELR_EL3.set(LR.get());
        }
        // Disable EL1 timer traps and the timer offset.
        CNTHCTL_EL2.modify(CNTHCTL_EL2::EL1PCEN::SET + CNTHCTL_EL2::EL1PCTEN::SET);
        CNTVOFF_EL2.set(0);
        // Set EL1 to 64bit.
        HCR_EL2.write(HCR_EL2::RW::EL1IsAarch64);
        // Set the return address and exception level.
        SPSR_EL2.write(
            SPSR_EL2::M::EL1h
                + SPSR_EL2::D::Masked
                + SPSR_EL2::A::Masked
                + SPSR_EL2::I::Masked
                + SPSR_EL2::F::Masked,
        );
        SP_EL1.set(SP.get());
        ELR_EL2.set(LR.get());
        aarch64_cpu::asm::eret();
    }
}

/// Enable EL1 MMU with the given page table root.
///
/// It usually used in the kernel booting process.
///
/// # Safety
///
/// This function is unsafe as it changes the address translation configuration.
pub unsafe fn enable_mmu(root_paddr: PhysAddr) {
    use page_table_entry::aarch64::MemAttr;

    MAIR_EL1.set(MemAttr::MAIR_VALUE);

    // Enable TTBR0 and TTBR1 walks, page size = 4K, vaddr size = 48 bits, paddr size = 48 bits.
    let tcr_flags0 = TCR_EL1::EPD0::EnableTTBR0Walks
        + TCR_EL1::TG0::KiB_4
        + TCR_EL1::SH0::Inner
        + TCR_EL1::ORGN0::WriteBack_ReadAlloc_WriteAlloc_Cacheable
        + TCR_EL1::IRGN0::WriteBack_ReadAlloc_WriteAlloc_Cacheable
        + TCR_EL1::T0SZ.val(16);
    let tcr_flags1 = TCR_EL1::EPD1::EnableTTBR1Walks
        + TCR_EL1::TG1::KiB_4
        + TCR_EL1::SH1::Inner
        + TCR_EL1::ORGN1::WriteBack_ReadAlloc_WriteAlloc_Cacheable
        + TCR_EL1::IRGN1::WriteBack_ReadAlloc_WriteAlloc_Cacheable
        + TCR_EL1::T1SZ.val(16);
    TCR_EL1.write(TCR_EL1::IPS::Bits_48 + tcr_flags0 + tcr_flags1);
    barrier::isb(barrier::SY);

    // Set both TTBR0 and TTBR1
    let root_paddr = root_paddr.as_usize() as u64;
    TTBR0_EL1.set(root_paddr);
    TTBR1_EL1.set(root_paddr);

    // Flush the entire TLB
    flush_tlb(None);

    // Enable the MMU and turn on I-cache and D-cache
    SCTLR_EL1.modify(SCTLR_EL1::M::Enable + SCTLR_EL1::C::Cacheable + SCTLR_EL1::I::Cacheable);
    barrier::isb(barrier::SY);
}

/// Enable FP and SIMD instructions.
///
/// It usually used in the kernel booting process.
pub fn enable_fp() {
    if cfg!(feature = "fp_simd") {
        CPACR_EL1.write(CPACR_EL1::FPEN::TrapNothing);
        barrier::isb(barrier::SY);
    }
}

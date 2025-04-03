use axhal_plat::init::InitIf;

#[cfg(feature = "irq")]
use crate::config::devices::{GICC_PADDR, GICD_PADDR, TIMER_IRQ, UART_IRQ};

struct InitIfImpl;

#[impl_plat_interface]
impl InitIf for InitIfImpl {
    /// Initializes the platform devices for the primary core.
    fn platform_init() {
        #[cfg(feature = "irq")]
        {
            use crate::mem::phys_to_virt;
            axplat_aarch64_common::gic::init_gicd(
                phys_to_virt(pa!(GICD_PADDR)),
                phys_to_virt(pa!(GICC_PADDR)),
            );
            axplat_aarch64_common::gic::init_gicc();
            axplat_aarch64_common::generic_timer::enable_irqs(TIMER_IRQ);

            // enable UART IRQs
            axhal_plat::irq::register(UART_IRQ, axplat_aarch64_common::pl011::irq_handler);
        }
    }

    /// Initializes the platform devices for secondary cores.
    fn platform_init_secondary() {
        #[cfg(all(feature = "smp", feature = "irq"))]
        {
            axplat_aarch64_common::gic::init_gicc();
            axplat_aarch64_common::generic_timer::enable_irqs(TIMER_IRQ);
        }
    }
}

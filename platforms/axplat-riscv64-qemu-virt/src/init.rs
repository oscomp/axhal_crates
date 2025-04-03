use axhal_plat::init::InitIf;

struct InitIfImpl;

#[impl_plat_interface]
impl InitIf for InitIfImpl {
    /// Initializes the platform devices for the primary CPU.
    ///
    /// For example, the interrupt controller and the timer.
    fn platform_init() {
        #[cfg(feature = "irq")]
        crate::irq::init_percpu();
        crate::time::init_percpu();
    }

    /// Initializes the platform devices for secondary CPUs.
    fn platform_init_secondary() {
        #[cfg(feature = "smp")]
        {
            #[cfg(feature = "irq")]
            crate::irq::init_percpu();
            crate::time::init_percpu();
        }
    }
}

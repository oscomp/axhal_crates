use axhal_plat::init::InitIf;

struct InitIfImpl;

#[impl_plat_interface]
impl InitIf for InitIfImpl {
    /// Initializes the platform devices for the primary CPU.
    ///
    /// For example, the interrupt controller and the timer.
    fn platform_init() {
        super::time::init_percpu();
    }

    /// Initializes the platform devices for secondary CPUs.
    fn platform_init_secondary() {
        #[cfg(feature = "smp")]
        {
            super::time::init_percpu();
        }
    }
}

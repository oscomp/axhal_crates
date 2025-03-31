use axhal_plat::init::InitIf;

struct InitIfImpl;

#[impl_plat_interface]
impl InitIf for InitIfImpl {
    /// Initializes the platform devices for the primary core.
    fn platform_init() {
        todo!()
    }

    /// Initializes the platform devices for secondary cores.
    fn platform_init_secondary() {
        todo!()
    }
}

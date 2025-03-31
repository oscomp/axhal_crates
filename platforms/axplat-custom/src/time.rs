use axhal_plat::time::TimeIf;

struct TimeIfImpl;

#[impl_plat_interface]
impl TimeIf for TimeIfImpl {
    /// Returns the current clock time in hardware ticks.
    fn current_ticks() -> u64 {
        todo!()
    }

    /// Converts hardware ticks to nanoseconds.
    fn ticks_to_nanos(_ticks: u64) -> u64 {
        todo!()
    }

    /// Converts nanoseconds to hardware ticks.
    fn nanos_to_ticks(_nanos: u64) -> u64 {
        todo!()
    }

    /// Return epoch offset in nanoseconds (wall time offset to monotonic
    /// clock start).
    fn epochoffset_nanos() -> u64 {
        todo!()
    }

    /// Set a one-shot timer.
    ///
    /// A timer interrupt will be triggered at the specified monotonic time
    /// deadline (in nanoseconds).
    fn set_oneshot_timer(_deadline_ns: u64) {
        todo!()
    }
}

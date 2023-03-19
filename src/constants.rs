use std::time::Duration;

pub const DEBUG_MODE: bool = cfg!(debug_assertions);

/// 30 second timer to submit an acronym.
pub const TIMER_DURATION: Duration = Duration::new(60, 0);

pub const DEBUG_DURATION: Duration = Duration::new(1_000_000, 0);

pub fn timer_duration() -> Duration {
    if DEBUG_MODE {
        DEBUG_DURATION
    } else {
        TIMER_DURATION
    }
}

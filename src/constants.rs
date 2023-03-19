use std::time::Duration;

/// 30 second timer to submit an acronym.
pub const TIMER_DURATION: Duration = Duration::new(60, 0);

pub const DEBUG_DURATION: Duration = Duration::new(1_000_000, 0);

pub fn timer_duration() -> Duration {
    if cfg!(debug_assertions) {
        DEBUG_DURATION
    } else {
        TIMER_DURATION
    }
}

use leptos::log;
use std::backtrace::Backtrace;
use std::fmt::Debug;

pub trait ResultExt<T> {
    fn ok_or_log(self) -> Option<T>;
}

impl<T, E: Debug> ResultExt<T> for Result<T, E> {
    /// Transorm a Result<T,E> into an `Option<T>`.
    /// Log the error if any.
    fn ok_or_log(self) -> Option<T> {
        match self {
            Ok(v) => Some(v),
            Err(e) => {
                // TODO: use the RUST_BACKTRACE env variable, and the regular `capture` function
                // instead, so this doesn't run in production.
                log!(
                    "logging error without panic: {:#?}\nbacktrace:\n{}",
                    e,
                    Backtrace::force_capture()
                );
                None
            }
        }
    }
}

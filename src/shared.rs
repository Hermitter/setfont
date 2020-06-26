use std::sync::atomic::{AtomicBool, Ordering};

/// State shared across all threads.
pub struct Shared {
    /// Indicates whether an error occurred in some thread.
    ///
    /// The process will terminate with a non-zero exit code if this is `true`
    /// when all work is completed.
    ///
    /// Do not set this value directly. Use `set_error` instead.
    pub did_error: AtomicBool,
}

impl Shared {
    pub fn new(did_error: bool) -> Self {
        Self {
            did_error: AtomicBool::new(did_error),
        }
    }

    /// Sets `did_error` to `true` with a correct memory ordering.
    pub fn set_error(&self) {
        self.did_error.store(true, Ordering::SeqCst);
    }
}

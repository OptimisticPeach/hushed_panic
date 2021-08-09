//!
//! A crate for hushing panics for a specific thread.
//!
//! This is especially useful for when you want to hush
//! tests which are intended to panic, but do not want
//! to see the long output.
//!
//! Usage in a test:
//! ```should_panic
//! fn my_test() {
//!     let _x = hushed_panic::hush_this_test();
//!     panic!(); // Won't print anything!
//!     drop(_x);
//!     panic!(); // Would print normally!
//! }
//! ```
//!

use once_cell::sync::OnceCell;
use std::thread::ThreadId;
use std::collections::HashSet;
use parking_lot::Mutex;
use std::panic::PanicInfo;
use std::marker::PhantomData;

static HUSHED_THREADS: OnceCell<Mutex<HashSet<ThreadId>>> = OnceCell::new();
static ORIGINAL_HANDLER: OnceCell<Box<dyn Fn(&PanicInfo) + Send + Sync + 'static>> = OnceCell::new();

/// Custom panic hook.
fn husher_hook(panic_info: &PanicInfo) {
    let thread_id = std::thread::current().id();
    if let Some(false) = HUSHED_THREADS.get().map(|x| {
        let guard = x.lock();
        guard.contains(&thread_id)
    }) {
        ORIGINAL_HANDLER.get().unwrap()(panic_info)
    }
}

/// Hushes panics for this thread.
pub fn hush_panic() {
    ORIGINAL_HANDLER.get_or_init(|| {
        let original = std::panic::take_hook();

        original
    });

    std::panic::set_hook(Box::new(husher_hook));

    let thread_id = std::thread::current().id();

    HUSHED_THREADS.get_or_init(Default::default).lock().insert(thread_id);
}

/// Returns whether the panic was hushed previously.
pub fn unhush_panic() -> bool {
    let thread_id = std::thread::current().id();

    HUSHED_THREADS.get_or_init(Default::default).lock().remove(&thread_id)
}

/// Returns a guard which will call `unhush_panic`
/// after it is dropped.
///
/// Use it as such:
/// ```norun
/// let _ = hush_this_test();
/// ```
pub fn hush_this_test() -> HushGuard {
    hush_panic();
    HushGuard { internal: PhantomData }
}

/// When this `struct` is dropped, the current thread's
/// panic is unhushed.
///
/// Create an instance of this by calling `hush_this_test`.
pub struct HushGuard { internal: PhantomData<*const ()> }

impl Drop for HushGuard {
    fn drop(&mut self) {
        unhush_panic();
    }
}

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

static HUSHED_THREADS: OnceCell<(Box<dyn Fn(&PanicInfo) + Send + Sync + 'static>, Mutex<HashSet<ThreadId>>)> = OnceCell::new();

/// Custom panic hook.
fn husher_hook(panic_info: &PanicInfo) {
    let thread_id = std::thread::current().id();

    while HUSHED_THREADS.get().is_none() {
        std::hint::spin_loop();
    }

    HUSHED_THREADS.get().map(move |(f, x)| {
        let guard = x.lock();
        if !guard.contains(&thread_id) {
            f(panic_info);
        }
    }).unwrap_or_else(|| println!("Something went wrong! Please report to `hushed_panic`'s github."));
}

fn init_hushed_threads() -> (Box<dyn Fn(&PanicInfo) + 'static + Send + Sync>, Mutex<HashSet<ThreadId>>) {
    let original = std::panic::take_hook();
    std::panic::set_hook(Box::new(husher_hook));

    (original, Default::default())
}

/// Hushes panics for this thread.
pub fn hush_panic() {
    let (_, threads) = HUSHED_THREADS.get_or_init(init_hushed_threads);

    let thread_id = std::thread::current().id();

    threads.lock().insert(thread_id);
}

/// Un-hushes panics on this thread.
///
/// Returns whether the panic was hushed previously.
pub fn unhush_panic() -> bool {
    let thread_id = std::thread::current().id();

    let val = HUSHED_THREADS.get_or_init(init_hushed_threads).1.lock().remove(&thread_id);

    val
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

//! <code>tokio = "[0.2](https://docs.rs/tokio/0.2/)"</code> ● **[spawn]**, [runtime], [try_set_runtime], [set_runtime], [handle]
#![cfg(feature = "v02")]

use tokio02 as tokio;
use tokio::runtime::{Builder, Handle, Runtime};
use tokio::task::JoinHandle;

use std::future::Future;
use std::sync::{Mutex, MutexGuard};

struct Common {
    used_runtime:   bool,
    runtime:        Option<Runtime>,
}

lazy_static::lazy_static! {
    static ref COMMON : Mutex<Common> = Mutex::new(Common{
        used_runtime:   false,
        runtime:        None,
    });

    static ref RUNTIME : Mutex<Runtime> = {
        let mut c = COMMON.lock().expect("unable to lock untokio::v02::COMMON");
        c.used_runtime = true;
        Mutex::new(c.runtime.take().unwrap_or_else(|| Builder::new()
            .enable_all()
            .threaded_scheduler()
            .thread_name("untokio::v02")
            .build()
            .expect("unable to create untokio::v02::RUNTIME")
        ))
    };

    static ref HANDLE : Handle = runtime().handle().clone();
}

/// Get a [Handle] for untokio's [Runtime]
pub fn handle() -> &'static Handle {
    &*HANDLE
}

/// Get a [Handle] for tokio - either the from tokio's [Handle::try_current] or from untokio's [handle]
pub fn current() -> Handle {
    Handle::try_current().unwrap_or_else(|_| HANDLE.clone())
}

/// Get untokio's [Runtime]
pub fn runtime() -> MutexGuard<'static, Runtime> {
    RUNTIME.lock().expect("unable to lock untokio::v02::RUNTIME")
}

/// Provide a [Runtime] instead of letting untokio create its own.
///
/// # Errors
///
/// * If the common lock is poisoned
/// * The runtime is already in use
pub fn try_set_runtime(runtime: Runtime) -> Result<(), &'static str> {
    let mut c = COMMON.lock().map_err(|_| "unable to lock untokio::v02::COMMON")?;
    if c.used_runtime { return Err("untokio::v02::RUNTIME already in use"); }
    c.runtime = Some(runtime);
    Ok(())
}

/// Provide a [Runtime] instead of letting untokio create its own.
///
/// # Panics
///
/// * If the common lock is poisoned
/// * The runtime is already in use
pub fn set_runtime(runtime: Runtime) {
    try_set_runtime(runtime).unwrap();
}

/// Spawns a new asynchronous task, returning a [JoinHandle] for it.
pub fn spawn<F>(future: F) -> JoinHandle<F::Output> where F : Future + Send + 'static, F::Output : Send + 'static {
    current().spawn(future)
}
